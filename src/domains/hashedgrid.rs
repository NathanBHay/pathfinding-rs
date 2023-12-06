//! A grid implementation which uses a hashset to represent obstacles
//! This is the simplest grid implementation and is fairly slow due
//! to the possibility of hashing collisions.

use std::{collections::{HashSet, HashMap}, fs::read_to_string};

use super::{create_map_from_string, print_cells, plot_cells};

/// A grid map whoch uses a hashset of obstacles to represent obstacles
/// This is the simplest grid map implementation
/// ## Fields
/// * `width` - The width of the grid map
/// * `height` - The height of the grid map
/// * `diagonal` - Whether or not diagonal movement is allowed
/// * `obstacles` - A hashset of obstacles
pub struct HashedGrid {
    pub width: usize,
    pub height: usize,
    pub obstacles: HashSet<usize>,
}

impl HashedGrid {
    /// Creates a new grid map with a given width and height
    /// ## Arguments
    /// * `width` - The width of the grid map
    /// * `height` - The height of the grid map
    /// ## Returns
    /// A new grid map with a given width and height
    pub fn new(width: usize, height: usize) -> HashedGrid {
        HashedGrid {
            width,
            height,
            obstacles: HashSet::new(),
        }
    }

    /// Creates a grid from string
    /// ## Arguments
    /// * `s` - A string representing the grid map
    /// ## Returns
    /// A grid map created from a string
    pub fn create_from_string(s: String) -> HashedGrid {
        let mut grid = create_map_from_string(s, HashedGrid::new, |grid, x, y| {
            grid.add_obstacle(x, y)
        });
        grid.invert();
        grid
    }

    /// Create a grid from a file
    /// ## Arguments
    /// * `filename` - The name of the file to read from
    /// ## Returns
    /// A grid map created from a file
    pub fn create_from_file(filename: &str) -> HashedGrid {
        let s = read_to_string(filename).expect("Unable to read file");
        HashedGrid::create_from_string(s)
    }

    /// Inverts the grid map
    /// ## Complexity
    /// O(n) where n is the number of cells in the grid map
    pub fn invert(&mut self) {
        let mut new_obstacles = HashSet::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get_map_value(x, y) {
                    new_obstacles.insert(x + y * self.width);
                }
            }
        }
        self.obstacles = new_obstacles;
    }

    /// Adds an obstacle to the grid map
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Complexity
    /// O(1)
    pub fn add_obstacle(&mut self, x: usize, y: usize) {
        self.obstacles.insert(x + y * self.width);
    }

    /// Removes an obstacle from the grid map
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Complexity
    /// O(1)
    pub fn remove_obstacle(&mut self, x: usize, y: usize) {
        self.obstacles.remove(&(x + y * self.width));
    }

    /// Checks if a given coordinate is valid to move into
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Returns
    /// True if the coordinate is an obstacle, false otherwise
    /// ## Complexity
    /// O(1)
    pub fn get_map_value(&self, x: usize, y: usize) -> bool {
        !self.obstacles.contains(&(x + y * self.width))
    }

    /// Checks if a given coordinate is valid
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Returns
    /// True if the coordinate is valid, false otherwise
    /// ## Complexity
    /// O(1)
    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Checks if a given coordinate is valid and not an obstacle
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Returns
    /// True if the coordinate is valid and not an obstacle, false otherwise
    /// ## Complexity
    /// O(1)
    pub fn valid_map_value(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.get_map_value(x, y)
    }

    /// Gets the neighbors of a given coordinate
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Returns
    /// A vector of coordinates that are neighbors of the given coordinate
    pub fn adjacent(&self, x: usize, y: usize, diagonal: bool) -> impl Iterator<Item = (usize, usize)> {
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
        neighbors.retain(|(x, y)| self.valid_map_value(*x, *y));
        neighbors.into_iter()
    }

    /// Prints the grid map where . is a free cell and @ is an obstacle
    pub fn print_cells(&self) -> String {
        print_cells(self.width, self.height, |x, y| self.get_map_value(x, y), None)
    }

    /// Prints the grid map where . is a free cell and @ is an obstacle
    pub fn print_cells_with_path(&self, path: Vec<(usize, usize)>) -> String {
        print_cells(self.width, self.height, |x, y| self.get_map_value(x, y), Some(path))
    }

    pub fn plot_cells(&self, filename: &str, path: Vec<(usize, usize)>) {
        plot_cells(self.width, self.height, filename, |x, y| self.get_map_value(x, y), Some(path), None)
    }

    pub fn plot_cells_with_heatmap(&self, filename: &str, heatmap: HashMap<(usize, usize), f64>) {
        plot_cells(self.width, self.height, filename, |x, y| self.get_map_value(x, y), None, Some(heatmap))
    }
    
}

#[cfg(test)]
mod tests {

    use super::HashedGrid;

    #[test]
    fn test_grid_add_obstacle() {
        let mut grid = HashedGrid::new(5, 5);
        grid.add_obstacle(0, 0);
        grid.add_obstacle(2, 3);
        assert_eq!(grid.get_map_value(0, 0), false);
        assert_eq!(grid.get_map_value(2, 3), false);
    }

    #[test]
    fn test_grid_create_from_string() {
        let map_str = ".....\n.@.@.\n.@.@.\n.@.@.\n.....\n....@\n";
        let grid = HashedGrid::create_from_string(map_str.to_string());
        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 6);
        assert_eq!(grid.get_map_value(0, 0), true);
        assert_eq!(grid.get_map_value(1, 1), false);
        assert_eq!(grid.get_map_value(3, 1), false);
        assert_eq!(grid.get_map_value(3, 3), false);
        assert_eq!(grid.get_map_value(4, 4), true);
        assert_eq!(grid.print_cells(), map_str);
    }

    #[test]
    fn test_grid_adjacent() {
        let grid = HashedGrid::create_from_string(String::from("...\n@.@\n@.@"));
        let neighbors = grid.adjacent(2, 1, true).collect::<Vec<_>>();
        assert_eq!(neighbors, vec![(1, 1), (2, 0), (1, 2), (1, 0)]);
    }
}