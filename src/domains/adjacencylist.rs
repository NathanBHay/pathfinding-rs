use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct AdjacencyList {
    pub nodes: Vec<Vec<(usize, i32)>>,
}

impl AdjacencyList {

    /// Creates a new adjacency list with a given size
    /// ## Arguments
    /// * `size` - The number of nodes in the graph
    /// ## Returns
    /// A new adjacency list with a given size
    pub fn new(size:usize) -> AdjacencyList {
        AdjacencyList {
            nodes: vec![Vec::new(); size],
        }
    }

    /// Adds a node to the graph
    /// ## Arguments
    /// * `node` - The node to add to the graph
    /// ## Panics
    /// Panics if the node already exists
    /// ## Complexity
    /// O(n) where n is the size of the node's adjacency list
    pub fn add_node(&mut self, node: Vec<(usize, i32)>) {
        for i in node.iter() {
            if i.0  > self.len() {
                panic!("Node does not exist");
            }
        }
        self.nodes.push(node);
    }

    /// Removes a node from the graph
    /// ## Arguments
    /// * `node` - The node to remove from the graph
    /// ## Panics
    /// Panics if the node does not exist
    /// ## Complexity
    /// O(n^2) where n is the number of nodes in the graph
    pub fn remove_node(&mut self, node: usize) {
        if node >= self.len() {
            panic!("Node does not exist");
        }

        self.nodes.remove(node);
        for i in 0..self.len() {
            self.nodes[i] = self
                .nodes[i]
                .iter()
                .filter(|&&x| x.0 != node)
                .map(|x| if x.0 > node {(x.0-1,x.1)} else {*x})
                .collect::<Vec<(usize, i32)>>();
        }
    }

    /// Removes a node from the graph, keeping it as empty vector
    /// ## Arguments
    /// * `node` - The node to remove from the graph
    /// ## Panics
    /// Panics if the node does not exist
    /// ## Complexity
    /// O(n) where n is the number of nodes in the graph
    /// ## Notes
    /// Used mainly in maze generation as to allow for printing of the maze.
    pub fn naive_remove_node(&mut self, node: usize) {
        if node >= self.len() {
            panic!("Node does not exist");
        }

        self.nodes[node] = Vec::new();
        for i in 0..self.len() {
            self.nodes[i].retain(|&(x, _)| x != node);
        }
    }

    /// Adds an edge between node1 and node2
    /// ## Arguments
    /// * `node1` - The outgoing node
    /// * `node2` - The incoming node
    /// ## Panics
    /// Panics if either node1 or node2 does not exist
    pub fn add_edge(&mut self, node1: usize, node2: usize) {
        self.add_edge_with_weight(node1, node2, 1)
    }

    /// Adds an edge with weight between node1 and node2 with weight
    /// ## Arguments
    /// * `node1` - The outgoing node
    /// * `node2` - The incoming node
    /// * `weight` - The weight of the edge
    /// ## Panics
    /// Panics if either node1 or node2 does not exist
    pub fn add_edge_with_weight(&mut self, node1: usize, node2: usize, weight: i32) {
        if node1 >= self.len() || node2 >= self.len() {
            panic!("Node does not exist");
        }
        self.nodes[node1].push((node2, weight));
    }

    /// Adds both edges between node1 and node2
    /// ## Arguments
    /// * `node1` - The first node
    /// * `node2` - The second node
    /// ## Panics
    /// Panics if either node1 or node2 does not exist
    pub fn add_edges(&mut self, node1: usize, node2: usize) {
        self.add_edges_with_weight(node1, node2, 1)
    }

    /// Adds an edge between node1 and node2
    /// ## Arguments
    /// * `node1` - The first node
    /// * `node2` - The second node
    /// * `weight` - The weight of the edge
    /// ## Panics
    /// Panics if either node1 or node2 does not exist
    pub fn add_edges_with_weight(&mut self, node1: usize, node2: usize, weight: i32) {
        self.add_edge_with_weight(node1, node2, weight);
        self.add_edge_with_weight(node2, node1, weight);
    }

    /// Remove an outgoing edge between node1 and node2
    /// ## Arguments
    /// * `node1` - Outgoing node
    /// * `node2` - Incoming Node
    /// ## Panics
    /// Panics if either node1 or node2 does not exist
    /// ## Complexity
    /// O(n) where n is the number of nodes in the graph
    pub fn remove_edge(&mut self, node1: usize, node2: usize) {
        if node1 >= self.len() || node2 >= self.len() {
            panic!("Node does not exist");
        }
        self.nodes[node1].retain(|&x| x.0 != node2);
    }

    /// Remove both edges between node1 and node2
    /// ## Arguments
    /// * `node1` - The first node
    /// * `node2` - The second node
    /// ## Panics
    /// Panics if either node1 or node2 does not exist
    pub fn remove_edges(&mut self, node1: usize, node2: usize) {
        self.remove_edge(node1, node2);
        self.remove_edge(node2, node1);
    }

    /// Remove all outgoing edges from a node
    /// ## Arguments
    /// * `node` - The node to remove all edges from
    /// ## Panics
    /// Panics if the node does not exist
    /// ## Complexity
    /// O(1) where n is the number of nodes in the graph
    pub fn remove_all_outgoing(&mut self, node: usize) {
        if node >= self.len() {
            panic!("Node does not exist");
        }
        self.nodes[node].clear();
    }

    /// Remove all incoming edges from a node
    /// ## Arguments
    /// * `node` - The node to remove all edges from
    /// ## Panics
    /// Panics if the node does not exist
    /// ## Complexity
    /// O(n^2) where n is the number of nodes in the graph
    pub fn remove_all_incoming(&mut self, node:usize) {
        if node >= self.len() {
            panic!("Node does not exist");
        }
        for i in 0..self.len() {
            self.nodes[i].retain(|&x| x.0 != node);
        }
    }

    /// Returns the number of nodes in the graph
    /// ## Returns
    /// The number of nodes in the graph
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Creates an iterator over the nodes in the graph
    /// ## Returns
    /// An iterator over the nodes in the graph
    pub fn iter(&self) -> Iter<Vec<(usize, i32)>> {
        self.nodes.iter()
    }

    /// Returns the nodes adjacent to a given node
    /// ## Arguments
    /// * `node` - The node to find the adjacent nodes of
    /// ## Returns
    /// A reference to the nodes adjacent to the given node
    pub fn adjacent(&self, node: usize) -> &Vec<(usize, i32)> {
        if node >= self.len() {
            panic!("Node does not exist");
        }
        &self.nodes[node]
    }

    /// Returns the degree of a given node
    /// ## Arguments
    /// * `node` - The node to find the degree of
    /// ## Returns
    /// The degree of the given node
    pub fn degree(&self, node: usize) -> usize {
        if node >= self.len() {
            panic!("Node does not exist");
        }
        self.nodes[node].len()
    }
}