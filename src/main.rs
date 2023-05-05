mod parsing;
mod board;
mod heuristic;
mod a_star;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	
	if args.len() != 2 {
		eprintln!("Error: Invalid args.");
		return; 
	}
	
	match parsing::parsing(&args[1]) {
		Ok((size, pieces)) => {
			let initial_board = board::Board::new(size, pieces);
			let final_board = board::final_board(&initial_board);

			println!("initial_board:");
			println!("{}", initial_board);

			println!("final_board:");
			println!("{}", final_board);

			//println!("{}", heuristic::manatthan_distance(&initial_board, &final_board));

			println!("{:?}", a_star::a_star(initial_board));
		},
		Err(err) => eprintln!("Error: {err}"),
	}
}