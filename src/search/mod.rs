#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash, ops::Add, cmp::Ordering};

pub mod uninformed;
pub mod astar;
pub mod idastar;
pub mod dstarlite;

/// Reconstructs a path from a given node to the start node
/// ## Arguments
/// * `parent` - A map of nodes to their parent nodes
/// * `node` - The node to reconstruct the path from
/// ## Returns
/// A vector of nodes from the start to the given node
pub(crate) fn reconstruct_path<N>(parent: HashMap<N, Option<N>>, mut node: N) -> Vec<N>
where
    N: Hash + Eq + Clone,
{
    let mut path = vec![node.clone()];
    while let Some(Some(prev)) = parent.get(&node) {
        path.push(prev.clone());
        node = prev.clone();
    }
    path.reverse();
    path
}

/// Reconstructs a path from a given node to the start node given a cost
/// ## Arguments
/// * `parent` - A map of nodes to their parent nodes
/// * `node` - The node to reconstruct the path from
/// ## Returns
/// A vector of nodes from the start to the given node and the cost of the path
pub(crate) fn reconstruct_path_with_cost<N, C>(
    parent: HashMap<N, (Option<N>, C)>,
    mut node: N,
) -> (Vec<N>, C)
where
    N: Hash + Eq + Clone,
    C: Ord + Default + Clone + Add<Output = C>,
{
    let mut path = vec![node.clone()];
    let mut cost = C::default();
    while let Some((Some(prev), c)) = parent.get(&node) {
        path.push(prev.clone());
        cost = cost + c.clone();
        node = prev.clone();
    }
    path.reverse();
    (path, cost)
}

/// Search node used in A-Star/D-Star Binary Heap
#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct SearchNodeState<N: Eq, C: Ord> {
    node: N,
    cost: C,
}

impl<N: Eq, C: Ord> Ord for SearchNodeState<N, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl<N: Eq, C: Ord> PartialOrd for SearchNodeState<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}