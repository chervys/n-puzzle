pub mod position;

use position::Position;
use position::Vector2D;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board {
    pub size: usize,
    pub value: Vec<usize>,
}

impl std::ops::Index<usize> for Board {
    type Output = usize;

    fn index(&self, i: usize) -> &Self::Output {
        &self.value[i]
    }
}
impl std::ops::Index<Position> for Board {
    type Output = usize;

    fn index(&self, i: Position) -> &Self::Output {
        &self.value[self.position_to_index(i)]
    }
}
impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.value[i]
    }
}
impl std::ops::IndexMut<Position> for Board {
    fn index_mut(&mut self, i: Position) -> &mut Self::Output {
        &mut self.value[Self::position_to_index_from_size(self.size, i)]
    }
}

impl Board {
    pub fn new(size: usize, pieces: Vec<usize>) -> Board {
        if size * size != pieces.len() {
            panic!(
                "Can't creat new board: len vector: {} size: {size}",
                pieces.len()
            );
        }

        let mut board = Board {
            size,
            value: Vec::new(),
        };

        for piece in pieces {
            board.value.push(piece)
        }

        board
    }

    pub fn get_hash(&self) -> String {
        let mut hash: String = String::new();
        for x in &self.value {
            let tmp = x.to_string();
            hash.push_str(&tmp);
        }
        hash
    }

    pub fn position_to_index(&self, position: Position) -> usize {
        position.x + position.y * self.size
    }

    pub fn position_to_index_from_size(size: usize, position: Position) -> usize {
        position.x + position.y * size
    }

    pub fn _index_to_position(&self, index: usize) -> Position {
        Position {
            x: index % self.size,
            y: index / self.size,
        }
    }

    pub fn _find_id(&self, id: usize) -> Option<(usize, usize)> {
        for (index, piece) in self.value.iter().enumerate() {
            if id == *piece {
                return Some((*piece, index));
            }
        }
        None
    }

    pub fn id_to_position(&self, id: usize) -> Option<Position> {
        match self.value.iter().position(|e| *e == id) {
            Some(index) => Some(self._index_to_position(index)),
            None => None,
        }
    }

    fn _move_piece(&mut self, piece_index: usize, hole_index: usize) {
        self.value.swap(piece_index, hole_index);
    }

    fn _find_adjacent_index(&self, index: usize) -> Vec<usize> {
        let mut adjacent = Vec::new();
        let size = self.size;

        if index + size < size * size {
            //bottom
            adjacent.push(index + size);
        }
        if index >= size {
            //top
            adjacent.push(index - size);
        }
        if index % size != 0 {
            //left
            adjacent.push(index - 1);
        }
        if (index + 1) % size != 0 {
            //right
            adjacent.push(index + 1);
        }
        adjacent
    }

    pub fn _derive(&self) -> Vec<Board> {
        let (_, hole_index) = self._find_id(0).expect("Missing hole");
        let mut derived = Vec::new();
        let adjacent_piece = self._find_adjacent_index(hole_index);

        for adjacent_index in adjacent_piece.iter() {
            let mut new_board = self.clone();
            new_board._move_piece(*adjacent_index, hole_index);
            derived.push(new_board);
        }
        derived
    }

    pub fn out_of_bound(&self, position: Position) -> bool {
        if position.x > self.size - 1 || position.y > self.size - 1 {
            return true;
        }
        false
    }

    pub fn assigned(&self, position: Position) -> bool {
        if self.value[self.position_to_index(position)] != 0 {
            return true;
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
        board.value.push(0)
    }

    board
}

pub fn final_board(board: &Board) -> Board {
    let mut direction = Vector2D { x: 1, y: 0 };
    let mut cursor = Position { x: 0, y: 0 };
    let mut final_board = blank_board(board.size);

    if board.size >= 1 {
        final_board[cursor] = 1;
        for i in 2..(board.size * board.size) {
            if final_board.out_of_bound(cursor + direction)
                || final_board.assigned(cursor + direction)
            {
                direction._rotate_right(); // rotate cursor
            }
            cursor = cursor + direction; // move cursor
            final_board[cursor] = i;
        }
    }

    final_board
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn new() {
        let pieces = vec![4, 1, 5, 3, 0, 6, 2, 8, 7];
        let size = 3;

        #[cfg(debug_assertions)]
        let board = Board::new(size, pieces);

        assert_eq!(board[0], 4);
        assert_eq!(board[8], 7);
        assert_eq!(board.size, 3);
    }
}
