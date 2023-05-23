use crate::board::position::Position;
use crate::board::Board;

pub fn manatthan_distance_for_piece(current_position: Position, final_position: Position) -> usize {
    current_position.x.abs_diff(final_position.x) + current_position.y.abs_diff(final_position.y)
}

pub fn manatthan_distance(current_board: &Board, final_board: &Board) -> usize {
    let mut length_of_road: usize = 0;

    for id in 0..(current_board.size * current_board.size) {
        if let (Some(a), Some(b)) = (
            current_board.id_to_position(id),
            final_board.id_to_position(id),
        ) {
            length_of_road += manatthan_distance_for_piece(a, b);
        }
    }

    length_of_road
}
