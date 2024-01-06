use std::collections::HashMap;
use std::collections::hash_map::ValuesMut;
use std::hash::Hash;
use std::ops::{Index, IndexMut};


#[derive(Debug, Clone)]
pub struct AdjacencyList<N: Eq + Hash, W>(HashMap<N, Vec<(N, W)>>);

impl<N: Eq + Hash, W> AdjacencyList<N, W> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn new_nodes(nodes: Vec<N>) -> Self {
        let mut map = HashMap::new();
        for node in nodes {
            map.insert(node, Vec::new());
        }
        Self(map)
    }

    pub fn add_node(&mut self, node: N) {
        self.0.insert(node, Vec::new());
    }

    pub fn add_edge(&mut self, from: N, to: N, weight: W) {
        self[from].push((to, weight));
    }

    pub fn remove_node(&mut self, node: N) {
        self.0.remove(&node);
        for (_, edges) in self.0.iter_mut() {
            edges.retain(|(n, _)| n != &node);
        }
    }

    pub fn remove_edge(&mut self, from: N, to: N) {
        self[from].retain(|(n, _)| n != &to);
    }

    pub fn adjacent(&self, node: N) -> impl Iterator<Item = &(N, W)> {
        self[node].iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn degree(&self, node: N) -> usize {
        self[node].len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&N, &Vec<(N, W)>)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> ValuesMut<N, Vec<(N, W)>> {
        self.0.values_mut()
    }
}

impl<N: Eq + Hash, W> Index<N> for AdjacencyList<N, W> {
    type Output = Vec<(N, W)>;
    fn index(&self, index: N) -> &Self::Output {
        &self.0[&index]
    }
}

impl<N: Eq + Hash, W> IndexMut<N> for AdjacencyList<N, W> {
    fn index_mut(&mut self, index: N) -> &mut Self::Output {
        self.0.get_mut(&index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_add() {
        let mut test = AdjacencyList::new();
        test.add_edge(1, 2, 1);
        assert_eq!(test[1], vec![(2, 1)]);
    }

    #[test]
    fn test_graph_remove_node() {
        let mut test = AdjacencyList::new_nodes((0..3).collect());
        test.add_edge(0, 2, 1);
        test.add_edge(1, 2, 1);
        test.remove_node(1);
        assert_eq!(test.len(), 2);
        assert_eq!(test[0], vec![(1, 1)]);
    }
}
