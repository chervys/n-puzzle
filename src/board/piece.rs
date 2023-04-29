use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub id: usize,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}