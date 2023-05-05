use std::{cmp::Ordering, collections::BTreeMap};
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
            .then_with(|| self.distance.cmp(&other.distance))
            .then_with(|| self.heuristic.cmp(&other.heuristic))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star(initial_board: Board) {
    let final_board: Board = board::final_board(&initial_board);

    let mut paths: BinaryHeap<Path> = BinaryHeap::new();

    let h: usize = heuristic::manatthan_distance(&initial_board, &final_board);
    paths.push(
        Path {
            board: initial_board.clone(),
            edges: Vec::new(),
            distance: 0,
            heuristic: h,
            cost: h,
        }
    );

	let mut _backup: BTreeMap<String, usize> = BTreeMap::new();

	let hash: String = initial_board.get_hash();
			match _backup.get(&hash) {
				Some(_a) => {
					if _a > &h {
						_backup.insert(hash, h);
					}
				}
				None => {
					_backup.insert(hash, h);
				}
			}

	let myMax = initial_board.size * initial_board.size * (initial_board.size - 1);

    while let Some(path) = paths.pop() {
	   //println!("{} {}", paths.len(), _backup.len());

        //if path.heuristic == 0 {
        //    println!("F({}) = G({}) + H({})", path.cost, path.distance, path.heuristic);
        //    println!("{:?}", path.edges);
        //    println!("{}", path.board);
        //    return;
        //}

        let new_boards: Vec<Board> = path.board._derive();

        for current_board in new_boards {

            let new_distance = path.distance + 1;
			if myMax < new_distance {
				continue;
			}

            let new_heuristic = heuristic::manatthan_distance(&current_board, &final_board);
            let new_cost = new_distance + new_heuristic;

			let hash: String = current_board.get_hash();
			match _backup.get(&hash) {
				Some(_a) => {
					if *_a > new_cost {
						_backup.insert(hash, new_cost);
					}
					else {
						continue;
					}
				}
				None => {
					_backup.insert(hash, new_cost);
				}
			}

			paths.push(
				Path {
                    board: current_board,
                    edges: path.edges.clone(),
                    distance: new_distance,
                    heuristic: new_heuristic,
                    cost: new_cost,
                }
            )
        }
    }

}