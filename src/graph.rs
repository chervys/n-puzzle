use crate::board::Board;

pub struct Node {
    pub board: Board,
    pub edges: Vec<usize>,
    pub parent: usize,
}

impl Node {

    pub fn new(board: Board) -> Node {
        Self { board, edges: Vec::new(), parent: usize::MAX }
    }

    pub fn add_edge(&mut self, index: usize) {
        self.edges.push(index);
    }

    pub fn derive_node(&mut self, parent: usize) -> Vec<Node> {
        let mut new_nodes = Vec::new();

        for board in self.board.derive() {
            new_nodes.push(Node{ board, edges:Vec::new(), parent });
        }
        new_nodes
    }
}

pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {

    pub fn new(board: Board) -> Graph {
        let mut graph = Graph {
            nodes: Vec::new(),
        };
        graph.add_node(Node::new(board));
        graph
    }

    pub fn add_node(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    pub fn add_nodes(&mut self, nodes: Vec<Node>) -> Vec<usize> {
        let mut index = Vec::new();
        for node in nodes {
            index.push(self.nodes.len());
            self.nodes.push(node);
        }
        index
    }

    pub fn derive_node(&mut self, node_index: usize) -> Vec<usize> {
        let new_nodes = self.nodes[node_index].derive_node(node_index);
        let new_edges = self.add_nodes(new_nodes);

        for edge in &new_edges {
            self.add_edge(node_index, *edge);
        }

        new_edges
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.nodes[source].add_edge(target);
    }
}