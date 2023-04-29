use std::env;

mod parsing;
mod board;
mod heuristic;
mod graph;

use board::Board;
use graph::Graph;

use crate::board::piece;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 2 {
		panic!("Invalid args.");
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

	println!("Initial:\n{board}\nDerived:");
	let mut graph = Graph::new(board);

	for i in graph.derive_node(0) {
		println!("{}", graph.nodes[i].board)
	}

	println!("Final board:\n{}", board::final_board(&graph[0].board));
}