mod domains;
mod heuristics;
mod mapf;
mod search;
mod util;

use std::collections::{BinaryHeap, HashMap};

use domains::bitpackedgrid::BitPackedGrid;
use search::uninformed::bfs;

use crate::domains::samplinggrid::SamplingGrid;
// use crate::search::astar::a_star;

fn main() {
    // let mut grid = SamplingGrid::create_from_string("...#...".to_string());
    // grid.conv_blur(2);
    // println!("{:?}", grid.sample_grid);

    let mut grid = SamplingGrid::create_from_file("map.map");
    // grid.sample_all();
    // grid.naive_blur(3);
    // let mut stored_grid = HashMap::new();

    // let epoch = 10;
    // for _ in 0..epoch {
    //     grid.sample_all();
    //     let path = bfs(
    //         |(x, y)| grid.adjacent(x.clone(), y.clone(), false), 
    //         (225, 225), 
    //         |p| { p == &(70, 40) },
    //     ).map(|(path, _)| path);
    //     if path.is_none() {
    //         continue;
    //     }
    //     for (x, y) in path.clone().unwrap() {
    //         if stored_grid.contains_key(&(x, y)) {
    //             stored_grid.insert((x, y), stored_grid[&(x,y)] + 1.0 / epoch as f64);
    //         } else {
    //             stored_grid.insert((x, y), 1.0 / epoch as f64);
    //         }
    //     }
    // }
    // grid.plot_cells_with_heatmap("sample.png", stored_grid);


    // let mut stored_grid = HashMap::new();
    // for _ in 0..epoch {
    //     let path = bfs(
    //         |(x, y)| grid.sampled_adjacent(x.clone(), y.clone(), false), 
    //         (225, 225), 
    //         |p| { p == &(70, 40) },
    //     ).map(|(path, _)| path);
    //     for (x, y) in path.clone().unwrap() {
    //         if stored_grid.contains_key(&(x, y)) {
    //             stored_grid.insert((x, y), stored_grid[&(x,y)] + 1.0 / epoch as f64);
    //         } else {
    //             stored_grid.insert((x, y), 1.0 / epoch as f64);
    //         }
    //     }
    // }
    // grid.plot_cells_with_heatmap("sample2.png", stored_grid);

    
    grid.plot_cells("samplingrid1.png", None);
    // grid.plot_cells("samplingrid2.png", None);q
    // grid.plot_cells("samplingrid.png", path.clone());
    // println!("{}", grid.print_cells());
    // print!("{}", grid.print_cells_with_path(path));
}
