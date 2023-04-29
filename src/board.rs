pub mod piece;
mod position;

use piece::Piece;
use position::Position;
use position::Vector2D;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    pub value: Vec<Piece>,
    pub size: usize,
}

impl std::ops::Index<usize> for Board {
    type Output = Piece;

    fn index(&self, i: usize) -> &Self::Output {
        &self.value[i]
    }
}
impl std::ops::Index<Position> for Board {
    type Output = Piece;

    fn index(&self, i: Position) -> &Self::Output {
        &self.value[self.position_to_index(i)]
    }
}
impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, i: usize) -> & mut Self::Output {
        &mut self.value[i]
    }
}
impl std::ops::IndexMut<Position> for Board {
    fn index_mut(&mut self, i: Position) -> & mut Self::Output {
        &mut self.value[Self::position_to_index_from_size(self.size, i)]
    }
}

impl Board {

    pub fn new(size: usize, pieces: Vec<usize>) -> Board {
        if size * size != pieces.len() {
            panic!("Can't creat new board: len vector: {} size: {size}", pieces.len());
        }

        let mut board = Board {
            size,
            value: Vec::new(),
        };

        for piece in pieces {
            board.value.push(Piece { id: piece })
        }

        board
    }

    pub fn position_to_index(&self, position: Position) -> usize {
        position.x + position.y * self.size
    }

    pub fn position_to_index_from_size(size: usize, position: Position) -> usize {
        position.x + position.y * size
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

    pub fn out_of_bound(&self, position: Position) -> bool {
        if position.x > self.size - 1 || position.y > self.size - 1 {
            return true
        }
        false
    }

    pub fn assigned(&self, position: Position) -> bool {
        if self.value[self.position_to_index(position)].id != 0 {
            return true
        }
        false
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

pub fn blank_board(size: usize) -> Board {
    let mut board = Board { 
        value: Vec::new(),
        size,
    };

    for _ in 0..size * size {
        board.value.push(Piece{
            id: 0,
        })
    }

    board
}

pub fn final_board(board: &Board) -> Board {
    let mut direction = Vector2D{x: 1, y: 0};
    let mut cursor = Position{x: 0, y: 0};
    let mut final_board = blank_board(board.size);

    if board.size >= 1 {
        final_board[cursor] = Piece{id: 1};
        for i in 2..(board.size * board.size) {
            if final_board.out_of_bound(cursor + direction) || final_board.assigned(cursor + direction) {
                direction.rotate_right(); // rotate cursor
            }
            cursor = cursor + direction; // move cursor
            final_board[cursor] = Piece{id: i};
        }
    }

    final_board
}

#[cfg(test)]
mod tests {
    use crate::Board;
    #[test]
    fn new() {
        let pieces = vec![4, 1, 5, 3, 0, 6, 2, 8, 7];
        let size = 3;

        #[cfg(debug_assertions)]
        let board = Board::new(size, pieces);

        assert_eq!(board[0].id, 4);
        assert_eq!(board[8].id, 7);
        assert_eq!(board.size, 3);
    }
}