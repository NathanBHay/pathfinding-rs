use domains::bitpackedgrid::BitPackedGrid;
use search::astar::astar;

mod domains;
mod heuristics;
mod search;
mod util;
mod fov;

fn main() {

    let grid = BitPackedGrid::create_from_file("basic.map");
    let path = astar(
        |(x, y)| grid.adjacent(x.clone(), y.clone(), false).map(|(x, y)| ((x, y), 1)), 
        (0, 0),
        |(x, y)| *x == 9 && *y == 9,
        |_| 0, 
    ).map(|(path, _)| path);
    // println!("{:?}", result);
    
    // grid.plot_cells("samplingrid1.png", None);
    // grid.plot_cells("samplingrid2.png", None);q
    grid.plot_cells("new.png", path.clone());
    // println!("{}", grid.print_cells());
    // print!("{}", grid.print_cells_with_path(path));
}



// fn run_sampling_grid_approach(epoch: u32) {
//     let mut grid = SamplingGrid::create_from_file("map.map");
//     grid.conv_blur(2);
//     let mut stored_grid = HashMap::new();
//     for _ in 0..epoch {
//         let path = bfs(
//             |(x, y)| grid.adjacent(x.clone(), y.clone(), false), 
//             (225, 225), 
//             |p| { p == &(70, 40) },
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