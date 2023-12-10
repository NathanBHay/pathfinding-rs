#![allow(dead_code)]
pub mod adjacencylist;
pub mod hashedgrid;
pub mod bitpackedgrid;
pub mod samplinggrid;

use std::collections::HashMap;

use plotters::{prelude::*, style::Color};
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
pub(crate) fn create_map_from_string<F, D, I>(
    s: String, 
    mut initialize: I, 
    mut add_obstacle: F
) -> D
where
    I: FnMut(usize, usize) -> D,
    F: FnMut(&mut D, usize, usize) -> (), 
{
    let s = s.trim();
    let start = s.find(|c| c == '.' || c == '@');
    let s = match start {
        Some(start) => &s[start..],
        None => "",
    };
    let height = s.lines().count();
    let width = s.lines().next().map(|x| x.len()).unwrap();
    let mut domain = initialize(width, height);
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
/// * `get_cell_value` - A function that returns the value of a cell given its x, y coordinates, 
/// true if the cell is free and false if it is an obstacle
/// ## Returns
/// A string representing the map where . is a free cell and @ is an obstacle
pub(crate) fn print_cells(
    width: usize, 
    height: usize, 
    get_cell_value: impl Fn(usize, usize) -> bool, 
    path: Option<Vec<(usize, usize)>>
) -> String {
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
    if let Some(path) = path {
        for (x, y) in path {
            s.replace_range(y * (width + 1) + x..y * (width + 1) + x + 1, "*");
        }
    }
    s
}

/// A debuging function which prints a vector of points
pub fn print_points(
    cells: Vec<(usize, usize)>
) -> String {
    let width_max = cells.iter().map(|(x, _)| x).max().unwrap() + 1;
    let width_min = cells.iter().map(|(x, _)| x).min().unwrap();
    let height_max = cells.iter().map(|(_, y)| y).max().unwrap() + 1;
    let height_min = cells.iter().map(|(_, y)| y).min().unwrap();
    print_cells(
        width_max - width_min, 
        height_max - height_min, 
        |x, y| cells.contains(&(x + width_min, y + height_min)), 
        None
    )
}

/// Helper function to plot the cells on a plotter backend
/// ## Arguments
/// * `width` - The width of the map
/// * `height` - The height of the map
/// * `output_file` - The output file to save the plot to
/// * `get_cell_value` - A function that returns the value of a cell given its x, y coordinates, 
/// true if the cell is free and false if it is an obstacle
/// * `path` - An optional path to plot on the map
/// * `heatmap` - An optional heatmap to plot on the map
pub(crate) fn plot_cells(
    width: usize, 
    height: usize, 
    output_file: &str, 
    get_cell_value: impl Fn(usize, usize) -> bool,
    path: Option<Vec<(usize, usize)>>,
    heatmap: Option<HashMap<(usize, usize), f64>>,
) {
    let root = BitMapBackend::new(output_file, (width as u32, height as u32)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(0..width as i32, 0..height as i32)
        .unwrap();
    chart.configure_mesh().disable_mesh().draw().unwrap();

    // Draw obstacles, will draw a rectangle for each cell that is an obstacle
    let series = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .filter(|(x, y)| !get_cell_value(x.clone(), y.clone()))
        .map(|(x, y)| (x as i32, (height-y) as i32))
        .map(|(x, y)| Rectangle::new([(x, y), (x + 1, y + 1)], &BLACK));
    chart.draw_series(series).expect("Unable to draw obstacles");

    // Draw path
    if let Some(path) = path {
        let path = path
            .iter()
            .map(|(x, y)| (*x as i32, (height - *y) as i32))
            .map(|(x, y)| Rectangle::new([(x, y), (x, y)], &RED));
        chart.draw_series(path).unwrap();
    }

    // Draw Heatmap of paths of values
    if let Some(heatmap) = heatmap {
        let heatmap = heatmap
            .iter()
            .map(|((x, y), color)| (*x as i32, (height - *y) as i32, *color))
            .map(|(x, y, color)| Rectangle::new([(x, y), (x, y)], &BLUE.mix(color)));
        chart.draw_series(heatmap).unwrap();
    }
}

/// Helper function to get a iterator of the neighbors of a cell
pub(crate) fn neighbors(
    x: usize, 
    y: usize, 
    diagonal: bool
) -> impl Iterator<Item = (usize, usize)>
{
    let mut neighbors = vec![
        (x.wrapping_add(1), y), 
        (x, y.wrapping_add(1)), 
        (x.wrapping_sub(1), y), 
        (x, y.wrapping_sub(1))
    ];
    if diagonal {
        neighbors.extend(vec![
            (x.wrapping_add(1), y.wrapping_add(1)),
            (x.wrapping_sub(1), y.wrapping_add(1)),
            (x.wrapping_add(1), y.wrapping_sub(1)),
            (x.wrapping_sub(1), y.wrapping_sub(1)),
        ]);
    }
    neighbors.into_iter()
}