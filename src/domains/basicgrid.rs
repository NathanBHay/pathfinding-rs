use std::collections::HashSet;


/// A grid map whoch uses a hashset of obstacles to represent obstacles
/// This is the simplest grid map implementation
/// ## Fields
/// * `width` - The width of the grid map
/// * `height` - The height of the grid map
/// * `diagonal` - Whether or not diagonal movement is allowed
/// * `obstacles` - A hashset of obstacles
#[repr(C)]
pub struct BasicGrid {
    pub width: usize,
    pub height: usize,
    diagonal: bool,
    pub obstacles: HashSet<usize>,
}

impl BasicGrid {

    /// Creates a new grid map with a given width and height
    /// ## Arguments
    /// * `width` - The width of the grid map
    /// * `height` - The height of the grid map
    /// ## Returns
    /// A new grid map with a given width and height
    pub fn new(width: usize, height: usize, diagonal: bool) -> BasicGrid {
        BasicGrid {
            width,
            height,
            diagonal,
            obstacles: HashSet::new(),
        }
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

    /// Checks if a given coordinate is an obstacle
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Returns
    /// True if the coordinate is an obstacle, false otherwise
    /// ## Complexity
    /// O(1)
    pub fn is_obstacle(&self, x: usize, y: usize) -> bool {
        self.obstacles.contains(&(x + y * self.width))
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
    pub fn is_valid_and_not_obstacle(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && !self.is_obstacle(x, y)
    }

    /// Gets the neighbors of a given coordinate
    /// ## Arguments
    /// * `x` - The x coordinate of the obstacle
    /// * `y` - The y coordinate of the obstacle
    /// ## Returns
    /// A vector of coordinates that are neighbors of the given coordinate
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
        if self.diagonal {
            neighbors.extend(vec![
                (x + 1, y + 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x - 1, y - 1),
            ]);
        }
        neighbors.iter()
            .map(|(x, y)| (*x, *y))
            .filter(|(x, y)| self.is_valid_and_not_obstacle(*x, *y))
            .collect::<Vec<_>>()
    }
}