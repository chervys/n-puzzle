
use crate::board::Board;
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
    dm
}

pub fn _manatthan_distance(_current_board: &Board, _final_board: &Board) -> usize {

    let mut dm: usize = 0;
    for index in 0.._current_board.size {

        println!("Current: ({}, {})\nFinal: ({}, {})", 
             _current_board._index_to_position(index).x,
             _current_board._index_to_position(index).y,
             _final_board._index_to_position(index).x,
             _final_board._index_to_position(index).y,
            );

        dm += _manatthan_distance_for_piece(
            _current_board._index_to_position(index), 
            _final_board._index_to_position(index)
        );
        println!("(Index, Dm): ({index}, {dm}) \n");
    }
    dm
}
