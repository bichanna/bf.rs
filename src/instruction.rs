#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Add(u8),
    Move(isize),
    Input,
    Output,
    JumpR(usize),
    JumpL(usize),
    Clear,
    AddTo(isize),
    MoveUntil(isize),
}
