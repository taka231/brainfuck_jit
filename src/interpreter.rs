use std::io::Read as _;

use crate::parser::{self, Instruction};

#[derive(Debug)]
pub struct Runtime {
    instructions: Vec<Instruction>,
    pc: usize,
    memory: Vec<u8>,
    // Data pointer
    dp: usize,
}

pub fn putchar(c: u8) {
    print!("{}", c as char);
}

pub fn readchar() -> u8 {
    let mut buffer = [0];
    match std::io::stdin().read_exact(&mut buffer) {
        Ok(_) => buffer[0],
        Err(e) => panic!("Error reading from stdin: {}", e),
    }
}

pub const MEMORY_SIZE: usize = 30000;

impl Runtime {
    pub fn new(program: &str) -> Self {
        let instructions = parser::parser(program);
        Self {
            instructions,
            pc: 0,
            memory: vec![0; MEMORY_SIZE],
            dp: MEMORY_SIZE / 2,
        }
    }

    pub fn run(&mut self) {
        use Instruction::*;
        loop {
            match self.instructions[self.pc] {
                PointerIncrement(n) => self.dp += n as usize,
                PointerDecrement(n) => self.dp -= n as usize,
                ValueIncrement(n) => self.memory[self.dp] = self.memory[self.dp].wrapping_add(n),
                ValueDecrement(n) => self.memory[self.dp] = self.memory[self.dp].wrapping_sub(n),
                PutChar => putchar(self.memory[self.dp]),
                ReadChar => self.memory[self.dp] = readchar(),
                LoopStart { end } => {
                    if self.memory[self.dp] == 0 {
                        self.pc = end;
                    }
                }
                LoopEnd { start } => {
                    if self.memory[self.dp] != 0 {
                        self.pc = start;
                    }
                }
                End => break,
            }
            self.pc += 1;
        }
    }
}
