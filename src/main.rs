use std::env;

mod parsing;
mod board;
mod heuristic;
mod graph;

use board::Board;
use graph::Graph;

use crate::heuristic::_manatthan_distance;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 2 {
		eprintln!("Error: Invalid args.");
		return; 
	}

	let path: String = args[1].clone();

	let mut pieces: Vec<usize> = Vec::new();
	let mut size: usize = 0;
	
	match parsing::parsing(path) {
		Ok((s, p)) => {
			size = s;
			pieces = p;
		},
		Err(err) => eprintln!("{err}"),
	}

	let initial_board = Board::new(size, pieces);
	let graph = Graph::new(initial_board.clone());
	let final_board = board::final_board(&graph.nodes[0].board);

	let current_board = graph.nodes[0].board.clone();

	println!("initial_board:");
	println!("{}", initial_board);

	println!("final_board:");
	println!("{}", final_board);

	println!("current_board:");
	println!("{}", current_board);

	println!("{}", _manatthan_distance(&initial_board, &final_board));

}