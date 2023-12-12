use std::collections::HashMap;

use domains::samplinggrid::SamplingGrid;
use heuristics::distance::manhattan_distance;
use search::astar::astar;

mod domains;
mod heuristics;
mod search;
mod util;
mod fov;
mod expansionpolicies;

fn main() {
    let mut grid = SamplingGrid::create_from_file("wall.map");
    grid.init_gridmap();
    grid.conv_blur(2); // Blur
    let mut current_node = (3, 0); // (225, 225);
    let mut final_path = Vec::new();
    let goal = (3, 6); //(70, 40); // [3][0] -> [3][6]
    let epoch = 10;
    let mut iteration = 0;
    while current_node != goal {
        final_path.push(current_node);
        let mut heatmap = HashMap::new();
        for _ in 0..epoch {
            grid.init_gridmap();
            grid.raycast_sample(current_node, 3);
            let path = astar(
                |(x, y)| grid.adjacent(*x, *y, false).map(|(x, y)| ((x, y), 1)).collect::<Vec<_>>(),
                current_node.clone(),
                |n| n == &goal,
                |(x, y)| manhattan_distance((*x, *y), (70, 40)) as usize, 
            ).map(|(path, _)| path);
            for (x, y) in path.clone().unwrap() {
                if heatmap.contains_key(&(x, y)) {
                    heatmap.insert((x, y), heatmap[&(x,y)] + 1.0);
                } else {
                    heatmap.insert((x, y), 1.0);
                }
            }
        }
        current_node = grid.adjacent(current_node.0, current_node.1, false)
            .map(|(x, y)| ((x, y), heatmap.get(&(x, y))))
            .max_by(|(_, v1), (_, v2)| {
                if v1.is_some_and(|x| x < &10.0) {
                    println!("Comparing {:?} and {:?}", v1, v2);
                }
                v1.partial_cmp(v2).unwrap_or(std::cmp::Ordering::Equal)})
            .map(|((x, y), _)| {
                (x, y)})
            .unwrap_or(current_node);
        let heatmap = heatmap.into_iter().map(|(k, v)| (k, v / epoch as f64)).collect::<Vec<_>>();
        // grid.plot_sampling_cells(format!("{}.png", iteration).as_str(), None, Some(heatmap));
        println!("Iteration: {}", iteration);
        println!("Iteration: {:?}", heatmap);
        iteration += 1;
    }
    grid.init_gridmap();
    grid.plot_cells("0final.png", Some(final_path), None)
}

/*
Expansion Functions:
Expand with radius 4 all equal cost:
|(x, y)| grid.kexpand((*x, *y), 4).map(|((x, y), _)| ((x, y), 1))



*/

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::{domains::bitpackedgrid::BitPackedGrid, search::astar::astar_with_expanded_set};

    use super::*;

    #[test]
    fn run_basic_search() {
        let grid = BitPackedGrid::create_from_file("map.map");
        let mut expanded_nodes = HashSet::new();
        let path = astar_with_expanded_set(
            |(x, y)| grid.adjacent(x.clone(), y.clone(), false).map(|(x, y)| ((x, y), 1)), 
            (225, 225),
            |n| n == &(70, 40),
            Some(&mut expanded_nodes),
            |(x, y)| manhattan_distance((*x, *y), (70, 40)), 
        ).map(|(path, _)| path);
        let expanded_nodes = expanded_nodes.iter().map(|n| (*n, 0.1)).collect::<Vec<_>>();
        grid.plot_cells("test1.png", path, Some(expanded_nodes));
    }

    #[test]
    fn run_better_search() {
        let grid = BitPackedGrid::create_from_file("map.map");
        let mut expanded_nodes = HashSet::new();
        let path = astar_with_expanded_set(
            |(x, y)| grid.raycast_expand((*x, *y), 4),
            (225, 225),
            |n| n == &(70, 40),
            Some(&mut expanded_nodes),
            |(x, y)| manhattan_distance((*x, *y), (70, 40)) as usize, 
        ).map(|(path, _)| path);
        let expanded_nodes = expanded_nodes.iter().map(|n| (*n, 0.1)).collect::<Vec<_>>();
        grid.plot_cells("test1.png", path, Some(expanded_nodes));
    }

    // #[test]
    // fn run_sampling_grid_approach(epoch: u32) {
    //     let mut grid = SamplingGrid::create_from_file("map.map");
    //     grid.conv_blur(2);
    //     let mut stored_grid = HashMap::new();
    //     for _ in 0..epoch {
    //         let path = astar(
    //             |(x, y)| grid.adjacent(x.clone(), y.clone(), false).map(move |(x, y)| ((x, y), 1)), 
    //             (225, 225), 
    //             |p| p == &(70, 40),
    //             |p| manhattan_distance((p.0 as i32, p.1 as i32), (70, 40)),
    //         ).map(|(path, _)| path);
    //         for (x, y) in path.clone().unwrap() {
    //             if stored_grid.contains_key(&(x, y)) {
    //                 stored_grid.insert((x, y), stored_grid[&(x,y)] + 1.0 / epoch as f64);
    //             } else {
    //                 stored_grid.insert((x, y), 1.0 / epoch as f64);
    //             }
    //         }
    //     }
    //     grid.plot_cells_with_heatmap("sample2.png", stored_grid);
    // }
}


