use crate::poker::range::Range

#[derive(Debug, Clone)]
pub struct Player {
    range: Range,
    stack_size: u16,
    current_bet: u16,
    position: Position,
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    InPosition,
    OutOfPosition,
}

impl Player {
    pub fn new(range: Range, stack_size: u16, position: Position) -> Self {
        Player {
            range,
            stack_size,
            position,
        }
    }
}
