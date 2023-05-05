use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::{board::Board, heuristic, board};

#[derive(Clone, Eq, PartialEq)]
pub struct Path {
    pub board: Board,
    pub edges: Vec<usize>,
    pub distance: usize, // G
    pub heuristic: usize, // H
    pub cost: usize, // F = G + H
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.heuristic.cmp(&other.heuristic))
            .then_with(|| self.distance.cmp(&other.distance))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn get_next_boards(board: Board ) -> Vec<Board> {    
    let next_boards: Vec<Board> = Vec::new();

    next_boards
}


pub fn a_star(initial_board: Board) {
    let final_board: Board = board::final_board(&initial_board);

    let mut paths: BinaryHeap<Path> = BinaryHeap::new();

    let h: usize = heuristic::manatthan_distance(&initial_board, &final_board);
    paths.push(
        Path {
            board: initial_board,
            edges: Vec::new(),
            distance: 0,
            heuristic: h,
            cost: h,
        }
    );

    while let Some(path) = paths.pop() {
        let _boards: Vec<Board> = get_next_boards(path.board);
    }

}