//! A series of uninformed search algorithms
//! These algorithms include:
//! * Breadth-First Search
//! * Depth-First Search
//! * Dijkstra's Algorithm

use std::{collections::{HashMap, VecDeque}, hash::Hash, ops::Add};

use super::reconstruct_path;
use crate::search::astar::a_star;

pub fn bfs<E, I, N, G>(expander: E, start: N, goal: G) -> Option<(Vec<N>, usize)>
where
    E: Fn(&N) -> I,
    I: IntoIterator<Item = N>,
    N: Hash + Eq + Clone,
    G: Fn(&N) -> bool,
{
    let mut queue = VecDeque::new();
    let mut distance = HashMap::new();
    distance.insert(start.clone(), None);
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if goal(&node) {
            let path = reconstruct_path(distance, node);
            let length = path.len() - 1;
            return Some((path, length));
        }
        for child in expander(&node) {
            if !distance.contains_key(&child) {
                distance.insert(child.clone(), Some(node.clone()));
                queue.push_back(child);
            }
        }
    }
    None
}

pub fn dls<E, I, N, G>(expander: E, start: N, goal: G, limit: usize) -> Option<(Vec<N>, usize)>
where
    E: Fn(&N) -> I,
    I: IntoIterator<Item = N>,
    N: Hash + Eq + Clone,
    G: Fn(&N) -> bool,
{
    let mut stack = Vec::new();
    let mut distance = HashMap::new();
    distance.insert(start.clone(), None);
    stack.push((start, 0));
    while let Some((node, depth)) = stack.pop() {
        if limit > depth {
            continue;
        }
        if goal(&node) {
            let path = reconstruct_path(distance, node);
            let length = path.len() - 1;
            return Some((path, length));
        }
        for child in expander(&node) {
            if !distance.contains_key(&child) {
                distance.insert(child.clone(), Some(node.clone()));
                stack.push((child, depth + 1));
            }
        }
    }
    None
}

pub fn iterative_deepening<E, I, N, G>(expander: E, start: N, goal: G) -> Option<(Vec<N>, usize)>
where
    E: Fn(&N) -> I,
    I: IntoIterator<Item = N>,
    N: Hash + Eq + Clone,
    G: Fn(&N) -> bool,
{
    for depth in 0.. {
        if let Some(result) = dls(&expander, start.clone(), &goal, depth) {
            return Some(result);
        }
    }
    None
}

pub fn dijkstra<H, C, E, I, N, G>(
    expander: E,
    start: N, 
    goal: G
) -> Option<(Vec<N>, C)>
where
    C: Ord + Default + Clone + Add<Output = C>,
    E: Fn(&N) -> I,
    I: IntoIterator<Item = (N, C)>,
    N: Hash + Eq + Clone,
    G: Fn(&N) -> bool,
{
    a_star(|_| C::default(), expander, start, goal)
}

#[cfg(test)]
mod tests {

    use crate::search::uninformed::bfs;

    #[test]
    fn test_bfs() {
        let results = bfs(|x| vec![x + 1, x + 2], 0, |x| *x == 2);
        assert_eq!(results.unwrap().0, vec![0, 2]);
    }

    #[test]
    fn test_bfs_no_result() {
        let results = bfs(|x| vec![x + 1, x + 2], 0, |x| *x == 3);
        assert_eq!(results, None);
    }
}
