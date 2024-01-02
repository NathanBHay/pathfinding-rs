// //! An A-Star Implementation with drop-in heuristic, expander, and domain.
// //! The implementation is similar to the approach used by 

use std::{hash::Hash, collections::{HashMap, BinaryHeap, HashSet}, ops::Add};
use super::{reconstruct_path_with_cost, SearchNodeState};



pub fn astar<E, I, C, N, G, H>(
    expander: E,
    start: N, 
    goal: G,
    heuristic: H, 
) -> Option<(Vec<N>, C)>
where
    E: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, C)>,
    C: Ord + Default + Clone + Add<Output = C>,
    N: Hash + Clone + Eq,
    G: Fn(&N) -> bool,
    H: Fn(&N) -> C,
{
    astar_with_expanded_set(expander, start, goal, None, heuristic)
}

/// A-Star Search
/// ## Arguments
/// * `expander` - A function that returns an iterator over the nodes adjacent to a given node
/// * `start` - The start node
/// * `goal` - A function that returns whether or not a given node is the goal
/// * `heuristic` - A function that returns the heuristic value of a given node
/// ## Returns
/// An optional vector of nodes from the start to the goal
pub fn astar_with_expanded_set<E, I, C, N, G, H>(
    mut expander: E,
    start: N, 
    goal: G,
    mut expanded_nodes: Option<&mut HashSet<N>>,
    heuristic: H, 
) -> Option<(Vec<N>, C)>
where
    E: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, C)>,
    C: Ord + Default + Clone + Add<Output = C>,
    N: Hash + Clone + Eq,
    G: Fn(&N) -> bool,
    H: Fn(&N) -> C,
{
    let mut open = BinaryHeap::new();
    let mut previous = HashMap::new();
    previous.insert(start.clone(), (None, C::default()));
    open.push(SearchNodeState {
        node: start.clone(),
        cost: heuristic(&start),
    });
    while let Some(SearchNodeState { node, .. }) = open.pop() {
        if let Some(expanded_nodes) = expanded_nodes.as_mut() {
            expanded_nodes.insert(node.clone());
        }
        if goal(&node) {
            return Some(reconstruct_path_with_cost(previous, node));
        }
        for (child, cost) in expander(&node) {
            let new_cost = previous[&node].1.clone() + cost;
            if !previous.contains_key(&child) || new_cost < previous[&child].1 {
                previous.insert(child.clone(), (Some(node.clone()), new_cost.clone()));
                open.push(SearchNodeState {
                    node: child.clone(),
                    cost: new_cost.clone() + heuristic(&child),
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{SearchNodeState, astar};
    use crate::domains::bitpackedgrid::BitPackedGrid;
    use std::collections::BinaryHeap;

    #[test]
    fn test_astar() {
        let results = astar(
            |x| vec![(x + 1, 1), (x + 2, 2)],
            0,
            |x| *x == 2,
            |x| *x,
        );
        assert_eq!(results.unwrap().0, vec![0, 2]);
    }

    #[test]
    fn test_astar_bitpacked_grid() {
        let grid = BitPackedGrid::new_from_string(".....\n.###.\n.#...\n.#.#.\n...#.".to_string());
        let path = astar(
            |(x, y)| grid.adjacent((x.clone(), y.clone()), false).map(|(x, y)| ((x, y), 1)), 
            (0, 4),
            |(x, y)| *x == 4 && *y == 4,
            |_| 0, 
        );
        assert_eq!(path.unwrap().0, vec![(0, 4), (1, 4), (2, 4), (2, 3), (2, 2), (3, 2), (4, 2), (4, 3), (4, 4)]);
    }

    #[test]
    fn test_astar_bitpacked_grid_with_heuristic() {
        let grid = BitPackedGrid::new_from_string("........\n...###..\n.....#..\n.....#..\n........\n........".to_string());
        let path = astar(
            |(x, y)| grid.adjacent((x.clone(), y.clone()), false).map(|(x, y)| ((x, y), 1)), 
            (0, 5),
            |(x, y)| *x == 7 && *y == 0,
            |_| 0, 
        );
        assert_eq!(path.unwrap().0, vec![(0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (6, 4), (7, 4), (7, 3), (7, 2), (7, 1), (7, 0)]);
    }

    #[test]
    fn test_search_node() {
        let mut open = BinaryHeap::new();
        open.push(SearchNodeState {
            node: 1,
            cost: 1,
        });
        open.push(SearchNodeState {
            node: 0,
            cost: 0,
        });
        open.push(SearchNodeState {
            node: 2,
            cost: 2,
        });
        assert_eq!(open.pop().unwrap().node, 0);
        assert_eq!(open.pop().unwrap().node, 1);
        assert_eq!(open.pop().unwrap().node, 2);
    }
}