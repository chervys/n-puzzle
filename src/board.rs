pub mod piece;
mod position;

use piece::Piece;
use position::Position;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    pub value: Vec<Piece>,
    pub size: usize,
}

impl Board {

    pub fn position_to_index(&self, position: Position) -> usize {
        position.x * self.size + position.y
    }

    pub fn index_to_position(&self, index: usize) -> Position {
        Position {
            x: index % self.size,
            y: index / self.size,
        }
    }

    pub fn find_id(&self, id: usize) -> Option<(&Piece, usize)> {
        for (index, piece) in self.value.iter().enumerate() {
            if id == piece.id {
                return Some((piece, index));
            }
        }
        None
    }

    fn move_piece(&mut self, piece_index: usize, hole_index: usize) {
        self.value.swap(piece_index, hole_index);
    }

    fn find_adjacent_index(&self, index: usize) -> Vec<usize> {
        let mut adjacent = Vec::new();
        let size = self.size;

        if index + size <= size * size { //bottom
            adjacent.push(index + size);
        }
        if index >= size { //top
            adjacent.push(index - size);
        }
        if index % size != 0 { //left
            adjacent.push(index - 1);
        }
        if (index + 1) % size != 0 { //right
            adjacent.push(index + 1);
        }
        adjacent
    }

    pub fn derive(&self) -> Vec<Board> {
        let (_, hole_index) = self.find_id(0).expect("Missing hole");
        let mut derived = Vec::new();
        let adjacent_piece = self.find_adjacent_index(hole_index);

        for adjacent_index in adjacent_piece.iter(){
            let mut new_board = self.clone();
            new_board.move_piece(*adjacent_index, hole_index);
            derived.push(new_board);
        }
        derived
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{} ", self.value[i * self.size + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}