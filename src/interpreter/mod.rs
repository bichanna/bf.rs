use crate::Instruction;

pub fn interpret() {
    let filename = std::env::args().nth(1).unwrap();
    let source = std::fs::read(filename).unwrap();
    Program::new(&source).run();
}

struct Program {
    pc: usize,
    ptr: usize,
    instructions: Vec<Instruction>,
    memory: [u8; 30000],
}

impl Program {
    fn new(source: &[u8]) -> Program {
        // Convert to Vec<Instruction>
        let instructions = source
            .into_iter()
            .filter_map(|b| match b {
                b'+' => Some(Instruction::Inc),
                b'-' => Some(Instruction::Dec),
                b'.' => Some(Instruction::Output),
                b',' => Some(Instruction::Input),
                b'>' => Some(Instruction::MoveR),
                b'<' => Some(Instruction::MoveL),
                b'[' => Some(Instruction::JumpR),
                b']' => Some(Instruction::JumpL),
                _ => None,
            })
            .collect::<Vec<Instruction>>();

        Program {
            pc: 0,
            ptr: 0,
            instructions,
            memory: [0; 30000],
        }
    }

    fn run(&mut self) {
        'program: loop {
            match self.instructions[self.pc] {
                Instruction::Inc => self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(1),
                Instruction::Dec => self.memory[self.ptr] = self.memory[self.ptr].wrapping_sub(1),
                Instruction::Output => print!("{}", self.memory[self.ptr] as char),
                Instruction::Input => {
                    use std::io::Read;
                    std::io::stdin()
                        .read_exact(&mut self.memory[self.ptr..self.ptr + 1])
                        .unwrap();
                }
                Instruction::MoveR => self.ptr = (self.ptr + 1) % self.memory.len(),
                Instruction::MoveL => {
                    self.ptr = (self.ptr + self.memory.len() - 1) % self.memory.len()
                }
                Instruction::JumpR => {
                    if self.memory[self.ptr] == 0 {
                        let mut deep = 1;
                        loop {
                            if self.instructions.len() == self.pc + 1 {
                                break 'program;
                            }
                            self.pc += 1;

                            if self.instructions[self.pc] == Instruction::JumpR {
                                deep += 1;
                            } else if self.instructions[self.pc] == Instruction::JumpL {
                                deep -= 1;
                            }

                            if deep == 0 {
                                break;
                            }
                        }
                    }
                }
                Instruction::JumpL => {
                    if self.memory[self.ptr] != 0 {
                        let mut deep = 1;
                        loop {
                            if self.pc == 0 {
                                break 'program;
                            }
                            self.pc -= 1;

                            if self.instructions[self.pc] == Instruction::JumpL {
                                deep += 1;
                            } else if self.instructions[self.pc] == Instruction::JumpR {
                                deep -= 1;
                            }

                            if deep == 0 {
                                break;
                            }
                        }
                    }
                }
                _ => {} // do nothing
            };

            self.pc += 1;

            if self.instructions.len() == self.pc {
                break 'program;
            }
        }
    }
}
