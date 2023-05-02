
use crate::board::{Board, position};
//use crate::board::piece::Piece;
use crate::board::position::Position;

pub fn _manatthan_distance_for_piece(_current_position: Position, _final_position: Position) -> usize {
    let mut dm:usize = 0;

    if _current_position.x >= _final_position.x {
        dm += _current_position.x - _final_position.x;
    }
    else {
        dm +=  _final_position.x - _current_position.x;
    }

    if _current_position.y > _final_position.y {
        dm += _current_position.y - _final_position.y;
    }
    else {
        dm +=  _final_position.y - _current_position.y;
    }
    println!("DM: {dm}");
    dm
}

pub fn _manatthan_distance(_current_board: &Board, _final_board: &Board) -> usize {

    let mut dm: usize = 0;
    for index in 0..(_current_board.size * _current_board.size) {

        let mut c: Position = Position {x: 0, y: 0};
        let mut f: Position = Position {x: 0, y: 0};
        if let Some(p) = _current_board.id_to_position(index) {c = p}
        if let Some(p) = _final_board.id_to_position(index) {f = p}

        println!("Current: ({}, {})\nFinal: ({}, {})", 
             c.x, c.y, f.x, f.y
            );

        dm += _manatthan_distance_for_piece(c, f);
        println!("(Index, Dm): ({index}, {dm}) \n");
    }
    dm
}
