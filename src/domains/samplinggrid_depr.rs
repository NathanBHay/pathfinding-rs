//! # Sampling Grid Map DEPRECATED
//! A grid map that has a grid which has precalculated probabilities of a cell
//! being traverseable, [0, 1] where 0 is an obstacle and 1 is free. This allows
//! for sampling of the cells to be done to determine if a cell is free or not.
//! Once sampled the cell is marked as to avoid repeated sampling.

use std::collections::HashMap;

use super::{create_map_from_string, print_cells, plot_cells, neighbors};
use rand;

/// A grid map that has a grid which has precalculated probabilities of a cell 
/// being free
pub struct SamplingGrid {
    pub sample_grid: Vec<Vec<f32>>,
    pub gridmap: HashMap<usize, bool>, // Represents a cell that has been sampled
    pub width: usize,
    pub height: usize,
}

impl SamplingGrid {

    /// Creates a new sampling grid from a sampling grid
    pub fn new_from_grid(grid: Vec<Vec<f32>>) -> SamplingGrid {
        let width = grid.len();
        let height = grid[0].len();
        SamplingGrid {
            sample_grid: grid,
            gridmap: HashMap::new(),
            width,
            height,
        }
    }

    /// Creates a new sampling grid with a given size
    pub fn new_with_size(width: usize, height: usize) -> SamplingGrid {
        SamplingGrid {
            sample_grid: vec![vec![0.0; height]; width],
            gridmap: HashMap::new(),
            width,
            height,
        }
    }

    /// Creates a sampling grid from a string
    pub fn new_from_string(map: String) -> SamplingGrid {
        create_map_from_string(map, SamplingGrid::new_with_size, |grid, x, y| {
            grid.sample_grid[x][y] = 1.0;
        })
    }

    /// Creates a sampling grid from a file
    pub fn new_from_file(filename: &str) -> SamplingGrid {
        let s = std::fs::read_to_string(filename).expect("Unable to read file");
        SamplingGrid::new_from_string(s)
    }

    pub fn init_gridmap(&mut self) {
        self.gridmap.drain();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.sample_grid[x][y] == 0.0 {
                    self.gridmap.insert(x * self.height + y, true);
                }
            }
        }
    }

    ///
    pub fn get_sample_grid_value(&self, x: usize, y: usize) -> usize {
        ((1.0 - self.sample_grid[x][y]) * 10.0) as usize
    }

    /// Returns whether a cell is an obstacle, true if not, otherwise false
    pub fn get_grid_value(&self, x: usize, y: usize) -> Option<bool> {
        self.gridmap.get(&(x * self.height + y)).map(|x| !x)
    }

    /// Sets a cell to be either an obstacle (false) or free (true)
    pub fn set_grid_value(&mut self, x: usize, y: usize, value: bool) -> bool {
        self.gridmap.insert(x * self.height + y, value);
        value
    }

    /// Checks if the cell is already known, if not checks chance of sampling,
    /// if it meets that it samples and uses that, if not it assumes that it
    /// is passible
    pub fn sample_with_chance(&mut self, x: usize, y: usize, chance: f32) -> bool {
        if let Some(val) = self.get_grid_value(x, y) {
            val
        } else if rand::random::<f32>() < chance {
            self.sample(x, y)
        } else {
            self.set_grid_value(x, y, true)
        }
    }

    /// Samples a cell, checks if its been sampled before and if not, samples it
    pub fn sample(&mut self, x: usize, y: usize) -> bool {
        if let Some(value) = self.get_grid_value(x, y) {
            value
        } else {
            let value = self.sample_grid[x][y] != 0.0 && rand::random::<f32>() < self.sample_grid[x][y];
            self.set_grid_value(x, y, value)
        }
    }

    /// Samples all cells in the grid, bad for space efficiency
    pub fn sample_all(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.sample(x, y);
            }
        }
    }

    pub fn conv_blur(&mut self, radius: usize) {
        let mut new_grid = vec![vec![1.0; self.height]; self.width];
        let mut kernel = vec![vec![1.0; radius * 2 + 1]; radius * 2 + 1];
        for i in 0..radius * 2 + 1 {
            for j in 0..radius * 2 + 1 {
                let dist = ((i as isize - radius as isize).pow(2) + (j as isize - radius as isize).pow(2)) as f32;
                kernel[i][j] = 1.0 / (dist + 1.0);
            }
        }
        for x in 0..self.width {
            for y in 0..self.height {
                if self.sample_grid[x][y] == 0.0 {
                    for i in 0..radius * 2 + 1 {
                        for j in 0..radius * 2 + 1 {
                            let x = x.wrapping_sub(radius).wrapping_add(i);
                            let y = y.wrapping_sub(radius).wrapping_add(j);
                            if self.bound_check(x, y) {
                                new_grid[x][y] = (new_grid[x][y] - kernel[i][j]).max(0.0);
                            }
                        }
                    }
                }
            }
        }
        self.sample_grid = new_grid;
    }
    
    /// Returns whether a cell is valid, sampling it if it is
    pub fn bound_check(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Checks if a cell is valid and a possible move. This also samples the cell
    /// resulting in future checks returning the same value
    pub fn valid_check(&mut self, x: usize, y: usize) -> bool {
        self.bound_check(x, y) && self.sample(x, y)
    }

    /// Returns the neighbors of a cell
    pub fn adjacent_sample(&mut self, x: usize, y: usize, diagonal: bool) -> impl Iterator<Item = (usize, usize)> + '_ {
        neighbors(x, y, diagonal).filter(move |(x, y)| self.valid_check(*x, *y))
    }

    pub fn adjacent(&self, x: usize, y: usize, diagonal: bool) -> impl Iterator<Item = (usize, usize)> + '_ {
        neighbors(x, y, diagonal).filter(move |(x, y)| self.bound_check(*x, *y) && self.get_grid_value(*x, *y).unwrap_or(true))
    }

    /// Prints the grid map where . is a free cell and @ is an obstacle
    pub fn print_cells(&self) -> String {
        self.print_cells_with_path(None)
    }

    pub fn print_sampling_cells(&self, path: Option<Vec<(usize, usize)>>) -> String {
        print_cells(self.width, self.height, |x, y| self.sample_grid[x][y] != 0.0, path)
    }

    pub fn print_cells_with_path(&self, path: Option<Vec<(usize, usize)>>) -> String {
        print_cells(self.width, self.height, |x, y| self.get_grid_value(x, y).map(|x| !x).unwrap_or(true), path)
    }

    pub fn plot_cells(&self, output_file: &str, path: Option<Vec<(usize, usize)>>, heatmap: Option<Vec<((usize, usize), f64)>>) {
        plot_cells(self.width, self.height, output_file, |x, y| self.get_grid_value(x, y).is_some_and(|x| !x), path, heatmap)
    }

    pub fn plot_sampling_cells(&self, output_file: &str, path: Option<Vec<(usize, usize)>>, heatmap: Option<Vec<((usize, usize), f64)>>) {
        plot_cells(self.width, self.height, output_file, |x, y| self.sample_grid[x][y] != 0.0, path, heatmap)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sampling_grid() {
        let mut grid = SamplingGrid::new_with_size(10, 10);
        grid.sample_grid[0][0] = 1.0;
        assert_eq!(grid.sample(0, 0), true);
    }

    #[test]
    fn test_create() {
        let grid = SamplingGrid::new_from_string("..@\n.@.\n".to_string());
        assert_eq!(grid.sample_grid[0][0], 1.0);
        assert_eq!(grid.sample_grid[2][0], 0.0);
        assert_eq!(grid.sample_grid[0][1], 1.0);
        assert_eq!(grid.sample_grid[2][1], 1.0);
    }

    #[test]
    fn test_sample() {
        let mut grid = SamplingGrid::new_from_string("..@\n.@.\n".to_string());
        assert_eq!(grid.sample(0, 0), true);
        assert_eq!(grid.sample(2, 0), false);
        assert_eq!(grid.sample(0, 1), true);
        assert_eq!(grid.sample(2, 1), true);
    }

    #[test]
    fn test_sampling_grid_blur() {
        let mut grid = SamplingGrid::new_from_string("...@..@".to_string());
        grid.conv_blur(2);
        assert_eq!(grid.sample_grid[0][0], 1.0);
        assert_eq!(grid.sample_grid[1][0], 0.8);
        assert_eq!(grid.sample_grid[2][0], 0.5);
        assert_eq!(grid.sample_grid[3][0], 0.0);
        assert_eq!(grid.sample_grid[4][0], 0.3);
        assert_eq!(grid.sample_grid[5][0], 0.3);
    }

    #[test]
    fn test_sampling_grid_print() {
        let map = ".....\n.@.@.\n.@.@.\n.@.@.\n.....\n....@\n".to_string();
        let mut grid = SamplingGrid::new_from_string(map.clone());
        grid.sample_all();
        assert_eq!(grid.print_cells(), map);
    }
}