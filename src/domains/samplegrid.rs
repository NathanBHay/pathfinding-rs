use super::bitpackedgrid::BitPackedGrid;
use super::{create_map_from_string, plot_cells, print_cells};

pub struct SampleGrid {
    /// The sampling grid which determines the probability of a cell being occupied.
    /// It has a value between 0.0 and 1.0
    pub sample_grid: Vec<Vec<f32>>,

    /// The bitpacked grid which represents sampled cells
    /// TODO: This cam be simplified to a smaller sub grid
    pub gridmap: BitPackedGrid,

    /// The real values of the grid
    pub ground_truth: BitPackedGrid,

    // The width and height of the grid
    // can be removed for reduced space
    pub width: usize,
    pub height: usize,
}

impl SampleGrid {

    /// Creates a new sampling grid from a sampling grid and a ground truth grid
    pub fn new_from_grid(grid: Vec<Vec<f32>>, ground_truth: BitPackedGrid) -> SampleGrid {
        let width = grid.len();
        let height = grid[0].len();
        let mut grid = SampleGrid {
            sample_grid: grid,
            gridmap: BitPackedGrid::new(width, height),
            ground_truth,
            width,
            height,
        };
        grid.init_gridmap();
        grid
    }

    /// Creates a new sampling grid with a given size
    pub fn new_with_size(width: usize, height: usize) -> SampleGrid {
        SampleGrid {
            sample_grid: vec![vec![0.0; height]; width],
            gridmap: BitPackedGrid::new(width, height),
            ground_truth: BitPackedGrid::new(width, height),
            width,
            height,
        }
    }

    /// Creates a sampling grid from a string
    pub fn create_from_string(map: String) -> SampleGrid {
        let mut grid = create_map_from_string(map, SampleGrid::new_with_size, |grid, x, y| {
            grid.sample_grid[x][y] = 1.0;
        });
        grid.init_gridmap();
        grid
    }

    /// Creates a sampling grid from a file
    pub fn create_from_file(filename: &str) -> SampleGrid {
        let s = std::fs::read_to_string(filename).expect("Unable to read file");
        SampleGrid::create_from_string(s)
    }

    /// Initializes an area of the bitfield from the sampling grid values, where
    /// 0.0 indicates a guaranteed obstacles and (0,1) indicates a probability
    fn init_gridmap_area(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.gridmap = BitPackedGrid::new(self.width, self.height);
        for x in x..x + width {
            for y in y..y + height {
                if self.sample_grid[x][y] != 0.0 {
                    self.gridmap.set_bit_value(x, y, true);
                }
            }
        }
    }

    /// Initializes the entire bitfield from the sampling grid values
    fn init_gridmap(&mut self) {
        self.init_gridmap_area(0, 0, self.width, self.height);
    }

    /// Samples a cell with a given chance
    pub fn sample(&mut self, x: usize, y: usize) {
        let value = self.sample_grid[x][y] != 0.0 && rand::random::<f32>() < self.sample_grid[x][y];
        self.gridmap.set_bit_value(x, y, value);
    }

    /// Samples all cells in the grid
    pub fn sample_all(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.sample(x, y);
            }
        }
    }

    /// Checks if within bounds
    fn bound_check(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn print_sampling_cells(&self, path: Option<Vec<(usize, usize)>>) -> String {
        print_cells(self.width, self.height, |x, y| self.sample_grid[x][y] != 0.0, path)
    }

    pub fn plot_sampling_cells(&self, output_file: &str, path: Option<Vec<(usize, usize)>>, heatmap: Option<Vec<((usize, usize), f64)>>) {
        plot_cells(self.width, self.height, output_file, |x, y| self.sample_grid[x][y] != 0.0, path, heatmap)
    }
}

#[cfg(test)]
mod tests {
    use super::SampleGrid;

    #[test]
    fn test_samplegrid_new() {
        let grid = SampleGrid::new_with_size(128, 128);
        assert_eq!(grid.height, 128);
        assert_eq!(grid.width, 128);
        assert_eq!(grid.sample_grid.len(), 128);
        assert_eq!(grid.sample_grid[0].len(), 128);
    }

    #[test]
    fn test_samplegrid_create() {
        let map_str = ".....\n@@.@.\n.@.@.\n.@.@.\n.....\n....@\n";
        let grid = SampleGrid::create_from_string(map_str.to_string());
        assert_eq!(grid.gridmap.print_cells(), map_str);
        assert_eq!(grid.sample_grid[0][0], 1.0);
        assert_eq!(grid.sample_grid[1][0], 1.0);
        assert_eq!(grid.sample_grid[0][1], 0.0);
        assert_eq!(grid.gridmap.get_bit_value(0, 0), true);
        assert_eq!(grid.gridmap.get_bit_value(1, 0), true);
        assert_eq!(grid.gridmap.get_bit_value(0, 1), false);
    }

}