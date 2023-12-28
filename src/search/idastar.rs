//! THIS CODE IS DEPRECATED AND WILL BE REMOVED
//! AS IT DOESN'T WORK

use std::{hash::Hash, collections::HashMap};


pub fn ida_star<E, I, N, G, H>(
    mut expander: E, 
    start: N, 
    goal: G, 
    heuristic: H
) -> Option<(Vec<N>, usize)>
where
    E: FnMut(&N) -> I,
    I: IntoIterator<Item = N>,
    N: Hash + Eq + Clone,
    G: Fn(&N) -> bool, 
    H: Fn(&N) -> usize,
{
    let mut threshold = heuristic(&start);
    loop {
        let (path, cost) = search(&mut expander, start.clone(), &goal, &heuristic, threshold);
        if let Some(path) = path {
            return Some((path, cost));
        }
        if cost == usize::max_value() {
            return None;
        }
        threshold = cost;
    }
}

/// Iterative Deepening Depth First Search Component
fn search<E, I, N, G, H>(mut expander: E, start: N, goal: G, heuristic: H, threshold: usize) -> (Option<Vec<N>>, usize)
where
    E: FnMut(&N) -> I,
    I: IntoIterator<Item = N>,
    N: Hash + Eq + Clone,
    G: Fn(&N) -> bool, 
    H: Fn(&N) -> usize,
{
    let mut stack = vec![(start.clone(), 0)];
    // let mut distance = HashMap::new();
    let mut path = vec![];
    // distance.insert(start, None);
    let mut min = usize::max_value();

    while let Some((node, cost)) = stack.pop() {
        if goal(&node) {
            // let path = reconstruct_path(distance, node);
            let length = path.len() - 1;
            return (Some(path), length);
        }
        if path.last() == Some(&node) {
            path.pop();
            continue;
        }
        path.push(node.clone());
        stack.push((node.clone(), cost));

        for child in expander(&node) {
            let f = cost + 1 + heuristic(&child);
            if f > threshold {
                min = min.min(f);
                continue;
            }
            if !path.contains(&child) {
                stack.push((child.clone(), cost + 1));
            }
            // if !distance.contains_key(&child) {
            //     distance.insert(child.clone(), Some(node.clone()));
            //     stack.push((child, cost + 1));
            // }
        }
    }
    (None, min)
}

#[cfg(test)]
mod tests {

    use crate::domains::hashedgrid::HashedGrid;
    use super::ida_star;

    #[test]
    fn test_a_star_bitpacked_grid() {
        let grid = HashedGrid::create_from_string(".....\n.###.\n.#...\n.#.#.\n...#.".to_string());
        let path = ida_star(
            |(x, y)| grid.adjacent(x.clone(), y.clone(), false),
            (0, 4),
            |(x, y)| *x == 4 && *y == 4,
            |_| 0, 
        );
        assert_eq!(path.unwrap().0, vec![(0, 4), (1, 4), (2, 4), (2, 3), (2, 2), (3, 2), (4, 2), (4, 3), (4, 4)]);
    }
}