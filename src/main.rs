mod board;

use board::Board;
use board::piece::Piece;
fn main() {
    let mut board = Board {
        value: Vec::new(),
        size: 3,
    };
    board.value.push(Piece { id: 4 });
    board.value.push(Piece { id: 1 });
    board.value.push(Piece { id: 2 });
    board.value.push(Piece { id: 3 });
    board.value.push(Piece { id: 0 });
    board.value.push(Piece { id: 5 });
    board.value.push(Piece { id: 6 });
    board.value.push(Piece { id: 7 });
    board.value.push(Piece { id: 8 });

    println!("Starting board :\n{board}");

    for (index, board) in board.derive().iter().enumerate() {
        println!("Derived board {index} :\n{board}");
    }
}