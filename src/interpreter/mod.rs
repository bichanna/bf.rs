use crate::Instruction::{self, *};

pub fn interpret(source: &[u8]) -> Result<(), UnbalancedBrackets> {
    Program::new(source)?.run();
    Ok(())
}

pub(crate) struct UnbalancedBrackets(pub char, pub usize);

struct Program {
    pc: usize,
    ptr: usize,
    instructions: Vec<Instruction>,
    memory: [u8; 30000],
}

impl Program {
    fn new(source: &[u8]) -> Result<Program, UnbalancedBrackets> {
        let mut instructions = Vec::new();
        let mut bracket_stack = Vec::new();

        for b in source {
            let inst = match b {
                b'+' => Inc,
                b'-' => Dec,
                b'.' => Output,
                b',' => Input,
                b'>' => MoveR,
                b'<' => MoveL,
                b'[' => {
                    let curr_addr = instructions.len();
                    bracket_stack.push(curr_addr);
                    JumpR(0)
                }
                b']' => {
                    let curr_addr = instructions.len();
                    match bracket_stack.pop() {
                        Some(pair_addr) => {
                            instructions[pair_addr] = JumpR(curr_addr);
                            JumpL(pair_addr)
                        }
                        None => return Err(UnbalancedBrackets(']', curr_addr)),
                    }
                }
                _ => continue,
            };
        }

        if let Some(unpaired_bracket) = bracket_stack.pop() {
            return Err(UnbalancedBrackets('[', unpaired_bracket));
        }

        Ok(Program {
            pc: 0,
            ptr: 0,
            instructions,
            memory: [0; 30000],
        })
    }

    fn run(&mut self) {
        'program: loop {
            match self.instructions[self.pc] {
                Inc => self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(1),
                Dec => self.memory[self.ptr] = self.memory[self.ptr].wrapping_sub(1),
                Output => print!("{}", self.memory[self.ptr] as char),
                Input => {
                    use std::io::Read;
                    std::io::stdin()
                        .read_exact(&mut self.memory[self.ptr..self.ptr + 1])
                        .unwrap();
                }
                MoveR => self.ptr = (self.ptr + 1) % self.memory.len(),
                MoveL => self.ptr = (self.ptr + self.memory.len() - 1) % self.memory.len(),
                JumpR(pair_addr) => {
                    if self.memory[self.ptr] == 0 {
                        self.pc = pair_addr;
                    }
                }
                JumpL(pair_addr) => {
                    if self.memory[self.ptr] != 0 {
                        self.pc = pair_addr;
                    }
                }
            };

            self.pc += 1;

            if self.instructions.len() == self.pc {
                break 'program;
            }
        }
    }
}
