#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    PointerIncrement(u32),
    PointerDecrement(u32),
    ValueIncrement(u8),
    ValueDecrement(u8),
    PutChar,
    ReadChar,
    LoopStart { end: usize },
    LoopEnd { start: usize },
    End,
}

pub fn parser(program: &str) -> Vec<Instruction> {
    use Instruction::*;
    let mut instrs = Vec::new();
    let mut jump_stack = Vec::new();
    for instr in program.chars() {
        match instr {
            '>' => {
                if let Some(PointerIncrement(n)) = instrs.last_mut() {
                    *n = n.wrapping_add(1);
                } else {
                    instrs.push(PointerIncrement(1));
                }
            }
            '<' => {
                if let Some(PointerDecrement(n)) = instrs.last_mut() {
                    *n = n.wrapping_add(1);
                } else {
                    instrs.push(PointerDecrement(1));
                }
            }
            '+' => {
                if let Some(ValueIncrement(n)) = instrs.last_mut() {
                    *n = n.wrapping_add(1);
                } else {
                    instrs.push(ValueIncrement(1));
                }
            }
            '-' => {
                if let Some(ValueDecrement(n)) = instrs.last_mut() {
                    *n = n.wrapping_add(1);
                } else {
                    instrs.push(ValueDecrement(1));
                }
            }
            '.' => instrs.push(PutChar),
            ',' => instrs.push(ReadChar),
            '[' => {
                instrs.push(LoopStart { end: 0 });
                jump_stack.push(instrs.len() - 1);
            }
            ']' => {
                let start = jump_stack.pop().unwrap();
                instrs[start] = LoopStart { end: instrs.len() };
                instrs.push(LoopEnd { start });
            }
            _ => (),
        }
    }
    instrs.push(End);
    instrs
}
