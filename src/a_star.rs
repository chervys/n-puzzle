use std::hash::Hash;
use std::{cmp::Ordering, collections::HashMap};
use std::collections::BinaryHeap;
use crate::{board::Board, heuristic, board, board::Move};

#[derive(Clone, Eq, PartialEq)]
pub struct Path {
    pub board: Board,
    pub edges: Vec<Move>,
    pub distance: usize, // G
    pub heuristic: usize, // H
    pub cost: usize, // F = G + H
}

impl Path {

    pub fn new(board: Board, final_board: &Board, edges: Vec<Move>) -> Self { //todo give heuristic
        let mut path = Path {
            board,
            edges,
            distance: 0,
            heuristic: 0,
            cost: 0,
        };
        let cost = path.cost(final_board);
        path.distance = cost.0;
        path.heuristic = cost.1;
        path.cost = cost.0 + cost.1;
        path
    }

    pub fn next(board: Board, final_board: &Board, edges: Vec<Move>, distance: usize) -> Self { //todo give heuristic
        let mut path = Path {
            board,
            edges,
            distance,
            heuristic: 0,
            cost: 0,
        };
        let cost = path.cost(final_board);
        path.heuristic = cost.1;
        path.cost = path.distance + path.heuristic;
        path
    }

    pub fn cost(&self, final_board: &Board) -> (usize, usize) { //todo give heuristic
        let new_distance = self.distance + 1;
        let new_heuristic = heuristic::manatthan_distance(&self.board, final_board);
        (new_distance, new_heuristic)
    }

    pub fn derive(&self, final_board: &Board) -> Vec<Path> { //todo give heuristic
        let mut paths = Vec::new();
        let new_boards = self.board._derive();

        for (current_board, _move) in new_boards {
            let previous_move = self.edges.last();
            if previous_move.is_none() || !previous_move.unwrap().is_opposit(_move) {
			    paths.push(Path::next(current_board, final_board, self.next_edges(_move), self.distance + 1))
            }
        }
        paths
    }

    pub fn next_edges(&self, _move: Move) -> Vec<Move> {
        let mut new_edges = self.edges.clone();
        new_edges.push(_move);
        new_edges
    }


}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.distance.cmp(&other.distance))
            .then_with(|| self.heuristic.cmp(&other.heuristic))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star(board: Board) {
    let final_board: Board = board::final_board(&board);

    let h: usize = heuristic::manatthan_distance(&board, &final_board);

	let mut _backup: HashMap<u64, usize> = HashMap::new();

    let initial_path = Path::new(board, &final_board, Vec::new());
    let mut paths: BinaryHeap<Path> = BinaryHeap::new();
    if _backup.get(&initial_path.board.get_hash()).is_none() { _backup.insert(initial_path.board.get_hash(), h); };
    paths.push(initial_path);


    while let Some(path) = paths.pop() {

        if path.heuristic == 0 {
            println!("FINAL_BOARD !");
            println!("F({}) = G({}) + H({})", path.cost, path.distance, path.heuristic);
            println!("{:?}", path.edges);
            println!("{}", path.board);
            return;
        }

        let new_paths = path.derive(&final_board);

        for path in new_paths {
            match _backup.get_mut(&path.board.get_hash()) {
                Some(v) => {
                    if *v > path.heuristic {
                        *v = path.heuristic;
                    }
                    continue;},
                None => {_backup.insert(path.board.get_hash(), path.heuristic);
                    // println!("F({}) = G({}) + H({})", path.cost, path.distance, path.heuristic);
                    // println!("{}", path.board);
                }
            }
            paths.push(path);
        }
    }
    println!("No soluce");
}