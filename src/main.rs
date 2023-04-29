mod board;
mod heuristic;
mod graph;

use board::Board;
use board::piece::Piece;
use graph::Graph;
use graph::Node;
fn main() {
    let pieces = vec![4, 1, 5, 3, 0, 6, 2, 8, 7];
    let size = 3;
    let board = Board::new(size, pieces);

    println!("Initial:\n{board}\nDerived:");
    let mut graph = Graph::new(board);

    for i in graph.derive_node(0) {
        println!("{}", graph.nodes[i].board)
    }

    println!("Final board:\n{}", board::final_board(&graph[0].board));
}