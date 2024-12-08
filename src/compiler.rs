use std::alloc;

use crate::{
    interpreter,
    parser::{self, Instruction},
};

#[derive(Debug)]
pub struct Compiler {
    instructions: Vec<Instruction>,
    jump_stack: Vec<*mut u8>,
    code_current: *mut u8,
    code_start: *mut u8,
}

const CODE_AREA_SIZE: usize = 4096 * 16;
const PAGE_SIZE: usize = 4096;

extern "C" {
    fn mprotect(addr: *const libc::c_void, len: libc::size_t, prot: libc::c_int) -> libc::c_int;
}

impl Compiler {
    pub unsafe fn new(program: &str) -> Self {
        let layout = alloc::Layout::from_size_align(CODE_AREA_SIZE, PAGE_SIZE).unwrap();
        let code_start = alloc::alloc(layout);
        let r = mprotect(
            code_start as *const libc::c_void,
            CODE_AREA_SIZE,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        );
        assert!(r == 0, "mprotect failed");

        Self {
            instructions: parser::parser(program),
            jump_stack: Vec::new(),
            code_current: code_start,
            code_start,
        }
    }

    unsafe fn emit_code(&mut self, code: &[u8]) {
        for byte in code {
            *self.code_current = *byte;
            self.code_current = self.code_current.add(1);
        }
        if self.code_current as usize - self.code_start as usize >= CODE_AREA_SIZE {
            panic!("Code area overflow");
        }
    }

    pub unsafe fn compile(&mut self) {
        use Instruction::*;
        // prologue
        // push rbp
        self.emit_code(&[0x50 + 5]);
        // mov rbp, rsp
        self.emit_code(&[0x48, 0x89, 0b11_100_101]);
        // push rbx
        self.emit_code(&[0x50 + 3]);
        // mov rbx, rdi
        self.emit_code(&[0x48, 0x89, 0b11_111_011]);
        // add rsp, -8
        self.emit_code(&[0x48, 0x83, 0b11_000_100, 0xf8]);

        for instr in self.instructions.clone() {
            match instr {
                // add rbx, (n)
                PointerIncrement(n) => {
                    if n > i32::MAX as u32 {
                        unimplemented!("n > i32::MAX");
                    }
                    self.emit_code(&[0x48, 0x81, 0b11_000_011]);
                    self.emit_code(&n.to_le_bytes());
                }
                // add rbx, (-n)
                PointerDecrement(n) => {
                    if n > -(i32::MIN as i64) as u32 {
                        unimplemented!("n > i32::MAX");
                    }
                    self.emit_code(&[0x48, 0x81, 0b11_000_011]);
                    self.emit_code(&(-(n as i32)).to_le_bytes());
                }
                // addb [rbx], (n)
                ValueIncrement(n) => self.emit_code(&[0x80, 0b00_000_011, n]),
                // addb [rbx], (-n)
                ValueDecrement(n) => {
                    self.emit_code(&[0x80, 0b00_000_011, (0xff - n).wrapping_add(1)])
                }
                PutChar => {
                    // mov dil, [rbx]
                    self.emit_code(&[0x40, 0x8a, 0b00_111_011]);
                    // mov r10, imm (address of putchar)
                    self.emit_code(&[0b0100_1001, 0xb8 + 2]);
                    self.emit_code(&(interpreter::putchar as *const () as u64).to_le_bytes());
                    // call r10
                    self.emit_code(&[0x41, 0xff, 0b11_010_010])
                }
                ReadChar => {
                    // mov r10, imm (address of readchar)
                    self.emit_code(&[0b0100_1001, 0xb8 + 2]);
                    self.emit_code(&(interpreter::readchar as *const () as u64).to_le_bytes());
                    // call r10
                    self.emit_code(&[0x41, 0xff, 0b11_010_010]);
                    // mov [rbx], al
                    self.emit_code(&[0x88, 0b00_000_011]);
                }
                LoopStart { .. } => {
                    // cmpb [rbx], 0
                    self.emit_code(&[0x80, 0b00_111_011, 0]);
                    // je 0 (dummy)
                    self.emit_code(&[0x0f, 0x84, 0, 0, 0, 0]);
                    self.jump_stack.push(self.code_current);
                }
                LoopEnd { .. } => {
                    // cmpb [rbx], 0
                    self.emit_code(&[0x80, 0b00_111_011, 0]);

                    let loop_start = self.jump_stack.pop().unwrap();
                    let offset = loop_start as i32 - (self.code_current as i32 + 6);
                    // jne imm (offset)
                    self.emit_code(&[0x0f, 0x85]);
                    self.emit_code(&offset.to_ne_bytes());

                    let offset = self.code_current as i32 - loop_start as i32;
                    for (i, byte) in offset.to_le_bytes().iter().enumerate() {
                        *loop_start.sub(4).add(i) = *byte;
                    }
                }
                End => {}
            }
        }

        // epilogue
        // add rsp, 8
        self.emit_code(&[0x48, 0x83, 0b11_000_100, 8]);
        // pop rbx
        self.emit_code(&[0x58 + 3]);
        // mov rsp, rbp
        self.emit_code(&[0x48, 0x89, 0b11_101_100]);
        // pop rbp
        self.emit_code(&[0x58 + 5]);
        // ret
        self.emit_code(&[0xc3]);
    }

    pub unsafe fn run(&self) {
        let f: fn(u64) = std::mem::transmute(self.code_start);
        let memory = vec![0; interpreter::MEMORY_SIZE];
        let dp = memory.as_ptr().add(interpreter::MEMORY_SIZE / 2) as u64;
        f(dp);
    }
}
