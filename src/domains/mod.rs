#![allow(dead_code)]
pub mod adjacencylist;
pub mod hashedgrid;
pub mod bitpackedgrid;
pub mod samplingrid;

/// A helper function that creates a map from a string given functions
/// to initialize the map and add obstacles
/// ## Arguments
/// * `s` - A string representing the grid map
/// * `initialize` - A function that initializes the map
/// * `add_obstacle` - A function that adds an obstacle to the map
/// ## Returns
/// A map with a domain equal to the return type of initialize
/// ## Complexity
/// (n) where n is the number of cells in the grid map, assuming initialize 
/// and add_obstacle are O(1)
pub(crate) fn create_map_from_string<F, D, I>(s: String, mut initialize: I, mut add_obstacle: F) -> D
where
    I: FnMut(usize, usize) -> D,
    F: FnMut(&mut D, usize, usize) -> (), 
{
    let s = s.trim();
    let height = s.lines().count();
    let width = s.lines().next().map(|x| x.len()).unwrap();
    let mut domain = initialize(height, width);
    for (i, line) in s.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                add_obstacle(&mut domain, j, i);
            }
        }
    }
    domain
}

/// A helper function that prints a map given a function to get the value of a cell
/// ## Arguments
/// * `width` - The width of the map
/// * `height` - The height of the map
/// * `get_cell_value` - A function that returns the value of a cell given its x, y coordinates
/// ## Returns
/// A string representing the map where . is a free cell and @ is an obstacle
pub(crate) fn print_cells<F: Fn(usize, usize) -> bool>(width: usize, height: usize, get_cell_value: F) -> String {
    let mut s = String::new();
    for y in 0..height {
        for x in 0..width {
            if get_cell_value(x, y) {
                s.push('.');
            } else {
                s.push('@');
            }
        }
        s.push('\n');
    }
    s
}