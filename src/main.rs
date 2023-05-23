mod astar;
mod board;
mod graph;
mod heuristic;
mod parsing;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Invalid args.");
        return;
    }

    match parsing::parsing(&args[1]) {
        Ok((size, pieces)) => {
            let initial_board = board::Board::new(size, pieces);
            let _graph = graph::Graph::new(initial_board.clone());
            println!("{:?}", astar::a_star(initial_board));
        }
        Err(err) => eprintln!("Error: {err}"),
    }
}
