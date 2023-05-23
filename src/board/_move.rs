#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn opposit(&self) -> Move {
        match self {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }

    pub fn is_opposit(&self, move_: Move) -> bool {
        *self == move_.opposit()
    }
}