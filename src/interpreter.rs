use std::io::Read as _;

#[derive(Debug)]
pub struct Runtime {
    instructions: Vec<char>,
    pc: usize,
    jump_stack: Vec<usize>,
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
        Self {
            instructions: program
                .chars()
                .filter(|c| ['>', '<', '+', '-', '.', ',', '[', ']'].contains(c))
                .collect(),
            pc: 0,
            jump_stack: Vec::new(),
            memory: vec![0; MEMORY_SIZE],
            dp: MEMORY_SIZE / 2,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.instructions.len() {
            match self.instructions[self.pc] {
                '>' => self.dp += 1,
                '<' => self.dp -= 1,
                '+' => self.memory[self.dp] = self.memory[self.dp].wrapping_add(1),
                '-' => self.memory[self.dp] = self.memory[self.dp].wrapping_sub(1),
                '.' => putchar(self.memory[self.dp]),
                ',' => self.memory[self.dp] = readchar(),
                '[' => {
                    if self.memory[self.dp] == 0 {
                        let mut bracket_count = 1;
                        while bracket_count != 0 {
                            self.pc += 1;
                            match self.instructions[self.pc] {
                                '[' => bracket_count += 1,
                                ']' => bracket_count -= 1,
                                _ => (),
                            }
                        }
                    } else {
                        self.jump_stack.push(self.pc);
                    }
                }
                ']' => {
                    if self.memory[self.dp] != 0 {
                        self.pc = *self.jump_stack.last().unwrap();
                    } else {
                        self.jump_stack.pop();
                    }
                }
                c => eprintln!("Invalid instruction: {c}"),
            }
            self.pc += 1;
        }
    }
}
