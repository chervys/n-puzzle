use std::env;

mod parsing;
mod board;
mod heuristic;
mod graph;

use board::Board;
use graph::Graph;

use crate::{heuristic::{_manatthan_distance, _manatthan_distance_for_piece}, board::position::Position};

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
	let board = Board::new(size, pieces);

	println!("Initial:\n{board}");
	let c: Board = board.clone();
	
	//println!("Derived:\n");
	let mut graph = Graph::new(board.clone());
	for _i in graph.derive_node(0) {
		//println!("{}", graph.nodes[_i].board)
	}

	println!("Final board:\n{}", board::final_board(&graph[0].board));

	//let c: Board = board.clone();
	let f: Board = board::final_board(&graph[0].board).clone();
	println!("{}", _manatthan_distance(
		&c, &c
	));

}