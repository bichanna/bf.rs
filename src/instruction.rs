#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Inc,
    Dec,
    MoveR,
    MoveL,
    Input,
    Output,
    JumpR,
    JumpL,
}
