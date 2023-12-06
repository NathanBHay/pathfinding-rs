// //! An A-Star Implementation with drop-in heuristic, queue, and domain
// //! 

use std::{cmp::Ordering, hash::Hash, collections::{HashMap, BinaryHeap}, ops::Add, fmt::Debug};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use super::reconstruct_path_with_cost;

/// Search node used in A-Star Binary Heap
#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchNodeState<N, C> {
    node: N,
    position: C,
}

impl<N: Eq, C: Ord> Ord for SearchNodeState<N, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.position.cmp(&other.position) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl<N: Eq, C: Ord> PartialOrd for SearchNodeState<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.position.cmp(&other.position))
    }
}

// /// A-Star Search
// /// ## Arguments
// /// * `heuristic` - A function that returns the heuristic value of a given node
// /// * `expander` - A function that returns an iterator over the nodes adjacent to a given node
// /// * `queue` - A queue to use for the search
// /// * `start` - The start node
// /// * `goal` - A function that returns whether or not a given node is the goal
// /// ## Returns
// /// An optional vector of nodes from the start to the goal
// pub fn a_star<E, I, C, N, G, H>(
//     mut expander: E,
//     start: N, 
//     goal: G,
//     heuristic: H, 
// ) -> Option<(Vec<N>, C)>
// where
//     E: FnMut(&N) -> I,
//     I: IntoIterator<Item = (N, C)>,
//     C: Ord + Default + Clone + Add<Output = C>,
//     N: Hash + Eq + Clone + Debug,
//     G: Fn(&N) -> bool,
//     H: Fn(&N) -> C,
// {
//     let mut open = BinaryHeap::new();
//     let mut previous: HashMap<N, (Option<N>, C)> = HashMap::new();
//     previous.insert(start.clone(), (None, C::default()));
//     open.push(SearchNodeState {
//         node: start.clone(),
//         position: heuristic(&start),
//     });
//     while let Some(SearchNodeState { node, .. }) = open.pop() {
//         if goal(&node) {
//             return Some(reconstruct_path_with_cost(previous, node));
//         }
//         for (child, cost) in expander(&node) {
//             let new_cost = previous[&node].1.clone() + cost;
//             if !previous.contains_key(&child) new_cost < previous[&child].1 {
//                 previous.insert(child.clone(), (Some(node.clone()), new_cost.clone()));
//                 open.push(SearchNodeState {
//                     node: child.clone(),
//                     position: new_cost.clone() + heuristic(&child),
//                 });
//             }
//         }
//     }
//     None
// }

// #[cfg(test)]
// mod tests {
//     use crate::search::astar::a_star;
//     use crate::domains::bitpackedgrid::BitPackedGrid;

//     #[test]
//     fn test_a_star() {
//         let results = a_star(
//             |x| vec![(x + 1, 1), (x + 2, 2)],
//             0,
//             |x| *x == 2,
//             |x| *x,
//         );
//         assert_eq!(results.unwrap().0, vec![0, 2]);
//     }

//     #[test]
//     fn test_a_star_no_result() {
//         let results = a_star(
//             |x| vec![(x + 1, 1), (x + 2, 2)],
//             0,
//             |x| *x == 3,
//             |x| *x,
//         );
//         assert_eq!(results, None);
//     }

//     #[test]
//     fn test_a_star_bitpacked_grid() {
//         let grid = BitPackedGrid::create_from_string(".....\n.###.\n.#...\n.#.#.\n...#.".to_string());
//         let path = a_star(
//             |(x, y)| grid.adjacent(x.clone(), y.clone(), false).map(|(x, y)| ((x, y), 1)), 
//             (0, 4),
//             |(x, y)| *x == 4 && *y == 4,
//             |_| 0, 
//         );
//         assert_eq!(path.unwrap().0, vec![(0, 4), (1, 4), (2, 4), (2, 3), (2, 2), (3, 2), (4, 2), (4, 3), (4, 4)]);
//     }

//     #[test]
//     fn test_a_star_bitpacked_grid_with_heuristic() {
//         let grid = BitPackedGrid::create_from_string("........\n...###..\n.....#..\n.....#..\n........\n........".to_string());
//         let path = a_star(
//             |(x, y)| grid.adjacent(x.clone(), y.clone(), false).map(|(x, y)| ((x, y), 1)), 
//             (0, 5),
//             |(x, y)| *x == 7 && *y == 0,
//             |_| 0, 
//         );
//         assert_eq!(path.unwrap().0, vec![(4, 4)]);
//     }
// }