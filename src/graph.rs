use crate::board::Board;

pub struct Node {
    pub board: Board,
    pub edges: Vec<usize>,
    pub parent: usize,
}

impl Node {
    pub fn new(board: Board) -> Node {
        Self {
            board,
            edges: Vec::new(),
            parent: usize::MAX,
        }
    }

    pub fn _add_edge(&mut self, index: usize) {
        self.edges.push(index);
    }

    pub fn _derive_node(&mut self, parent: usize) -> Vec<Node> {
        let mut new_nodes = Vec::new();

        for board in self.board._derive() {
            new_nodes.push(Node {
                board,
                edges: Vec::new(),
                parent,
            });
        }
        new_nodes
    }
}

pub struct Graph {
    pub nodes: Vec<Node>,
}

impl std::ops::Index<usize> for Graph {
    type Output = Node;

    fn index(&self, i: usize) -> &Self::Output {
        &self.nodes[i]
    }
}
impl std::ops::IndexMut<usize> for Graph {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.nodes[i]
    }
}

impl Graph {
    pub fn new(board: Board) -> Graph {
        let mut graph = Graph { nodes: Vec::new() };
        graph.add_node(Node::new(board));
        graph
    }

    pub fn add_node(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    pub fn _add_nodes(&mut self, nodes: Vec<Node>) -> Vec<usize> {
        let mut index = Vec::new();
        for node in nodes {
            index.push(self.nodes.len());
            self.nodes.push(node);
        }
        index
    }

    pub fn _derive_node(&mut self, node_index: usize) -> Vec<usize> {
        let new_nodes = self.nodes[node_index]._derive_node(node_index);
        let new_edges = self._add_nodes(new_nodes);

        for edge in &new_edges {
            self._add_edge(node_index, *edge);
        }

        new_edges
    }

    pub fn _add_edge(&mut self, source: usize, target: usize) {
        self.nodes[source]._add_edge(target);
    }
}

// mod board;
// mod heuristic;
// mod graph;

// use board::Board;
// use board::piece::Piece;
// use graph::Graph;
// use graph::Node;
// fn main() {
//     let pieces = vec![4, 1, 5, 3, 0, 6, 2, 8, 7];
//     let size = 3;
//     let board = Board::new(size, pieces);

//     println!("Initial:\n{board}\nDerived:");
//     let mut graph = Graph::new(board);

//     for i in graph.derive_node(0) {
//         println!("{}", graph.nodes[i].board)
//     }

//     println!("Final board:\n{}", board::final_board(&graph[0].board));
// }
