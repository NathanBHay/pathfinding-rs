use std::collections::HashMap;

use super::{create_map_from_string, print_cells, print_cells_with_path, plot_cells, plot_cells_with_heatmap, hashedgrid::HashedGrid};
use rand;

pub struct SamplingGrid {
    pub sample_grid: Vec<Vec<f32>>,
    pub grid: HashedGrid,
    pub width: usize,
    pub height: usize,
}

impl SamplingGrid {

    /// Creates a new sampling grid from a float vectors
    pub fn new_from_grid(grid: Vec<Vec<f32>>) -> SamplingGrid {
        let width = grid.len();
        let height = grid[0].len();
        SamplingGrid {
            sample_grid: grid,
            grid: HashedGrid::new(width, height),
            width,
            height,
        }
    }

    /// Creates a new sampling grid with a given size
    /// ## Arguments
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    /// ## Returns
    /// A new sampling grid with a given size
    pub fn new_with_size(width: usize, height: usize) -> SamplingGrid {
        SamplingGrid {
            sample_grid: vec![vec![0.0; height]; width],
            grid: HashedGrid::new(width, height),
            width,
            height,
        }
    }

    /// Creates a sampling grid from a string
    /// ## Arguments
    /// * `map` - A string representing the map where . is a free cell
    /// ## Returns
    /// A sampling grid created from a string
    pub fn create_from_string(map: String) -> SamplingGrid {
        let mut grid = create_map_from_string(map, SamplingGrid::new_with_size, |grid, x, y| {
            grid.sample_grid[x][y] = 1.0;
            grid.grid.add_obstacle(x, y);
        });
        grid.grid.invert();
        grid
    }

    /// Creates a sampling grid from a file
    /// ## Arguments!
    /// * `filename` - The name of the file to read from
    /// ## Returns
    /// A sampling grid created from a file
    pub fn create_from_file(filename: &str) -> SamplingGrid {
        let s = std::fs::read_to_string(filename).expect("Unable to read file");
        SamplingGrid::create_from_string(s)
    }

    pub fn sample(&mut self, x: usize, y: usize) -> bool {
        if self.sample_grid[x][y] != 0.0 && rand::random::<f32>() < self.sample_grid[x][y] {
            self.grid.remove_obstacle(x, y);
            true
        } else {
            self.grid.add_obstacle(x, y);
            false
        }
    }

    pub fn sample_all(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.sample(x, y);
            }
        }
    }

    // pub fn blur(&mut self, radius: usize) {
    //     let mut new_grid = vec![vec![0.0; self.height]; self.width];
    //     for x in 0..self.width {
    //         for y in 0..self.height {
    //             if self.grid[x][y] == 1.0 {
    //                 for i in x.saturating_sub(radius)..x.saturating_add(radius) {
    //                     for j in y.saturating_sub(radius)..y.saturating_add(radius) {
    //                         if self.is_valid(i, j)  {
    //                             new_grid[i][j] = 1.0;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     self.grid = new_grid;
    // }

    /// Blurs the grid by a given radius. This sets cells that are within the radius 
    /// of an adjacent to walls to 0.5
    pub fn naive_blur(&mut self, radius: usize) {
        let mut new_grid = self.sample_grid.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.sample_grid[x][y] == 0.0 {
                    for i in x.saturating_sub(radius.saturating_sub(1))..x.saturating_add(radius) {
                        for j in y.saturating_sub(radius.saturating_sub(1))..y.saturating_add(radius) {
                            if self.is_valid(i, j) && self.sample_grid[i][j] == 1.0 {
                                new_grid[i][j] = 0.5;
                            }
                        }
                    }
                }
            }
        }
        self.sample_grid = new_grid;
    }

    /// Returns whether a cell is valid, sampling it if it is
    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn is_valid_no_obstacle(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.grid.get_map_value(x, y)
    }

    pub fn is_valid_with_sampling(&mut self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.sample(x, y)
    }

    /// Returns the neighbors of a cell
    pub fn adjacent(&mut self, x: usize, y: usize, diagonal: bool) -> impl Iterator<Item = (usize, usize)> {
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
        neighbors.retain(|(x, y)| self.is_valid_no_obstacle(*x, *y));
        neighbors.into_iter()
    }

    pub fn sampled_adjacent(&mut self, x: usize, y: usize, diagonal: bool) -> impl Iterator<Item = (usize, usize)> {
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
        neighbors.retain(|(x, y)| self.is_valid_with_sampling(*x, *y));
        neighbors.into_iter()
    }

    /// Prints the grid map where . is a free cell and @ is an obstacle
    pub fn print_cells(&self) -> String {
        self.grid.print_cells()
    }

    pub fn print_cells_with_path(&self, path: Vec<(usize, usize)>) -> String {
        self.grid.print_cells_with_path(path)
    }

    pub fn plot_cells(&self, output_file: &str, path: Option<Vec<(usize, usize)>>) {
        self.grid.plot_cells(output_file, path)
    }

    pub fn plot_cells_with_heatmap(&self, output_file: &str, heatmap: HashMap<(usize, usize), f64>) {

        plot_cells_with_heatmap(self.width, self.height, output_file, |x, y| self.sample_grid[x][y] != 0.0, heatmap)
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
}