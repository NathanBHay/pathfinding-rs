use super::bitpackedgrid::BitPackedGrid;
use super::{create_map_from_string, plot_cells, print_cells};
use crate::util::matrix::{convolve2d, ConvResolve, gaussian_kernal, matrix_overlay};
use crate::util::filter::KalmanNode;

pub struct SampleGrid {
    /// The sampling grid which determines the probability of a cell being occupied.
    /// It has a value between 0.0 and 1.0
    pub sample_grid: Vec<Vec<KalmanNode>>,

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
    /// The default covariance of the Kalman filter
    const COVARIANCE: f32 = 1.0;

    /// Creates a new sampling grid from a sampling grid and a ground truth grid
    pub fn new_from_grid(grid: Vec<Vec<f32>>, ground_truth: BitPackedGrid) -> Self {
        let width = grid.len();
        let height = grid[0].len();
        let sample_grid = grid.into_iter()
            .map(|row| row.into_iter().map(|state| KalmanNode {
                state,
                covariance: Self::COVARIANCE,
            }).collect())
            .collect();
        let mut grid = SampleGrid {
            sample_grid,
            gridmap: BitPackedGrid::new(width, height),
            ground_truth,
            width,
            height,
        };
        grid.init_gridmap();
        grid
    }

    /// Creates a new sampling grid with a given size
    pub fn new_with_size(width: usize, height: usize) -> Self {
        let node = KalmanNode {
            state: 0.0,
            covariance: Self::COVARIANCE,
        };
        SampleGrid {
            sample_grid: vec![vec![node; height]; width],
            gridmap: BitPackedGrid::new(width, height),
            ground_truth: BitPackedGrid::new(width, height),
            width,
            height,
        }
    }

    /// Creates a sampling grid from a string
    pub fn new_from_string(map: String) -> Self {
        let mut grid = create_map_from_string(map, SampleGrid::new_with_size, |grid, x, y| {
            grid.sample_grid[x][y].state = 1.0;
        });
        grid.init_ground_truth();
        grid
    }

    /// Creates a sampling grid from a file
    pub fn new_from_file(filename: &str) -> Self {
        let s = std::fs::read_to_string(filename).expect("Unable to read file");
        SampleGrid::new_from_string(s)
    }
    
    /// Initializes an area of the bitfield from the sampling grid values, where
    /// 0.0 indicates a guaranteed obstacles and (0,1) indicates a probability
    pub fn init_gridmap_area(&mut self, (x, y): (usize, usize), width: usize, height: usize) {
        for x in x..x + width {
            for y in y..y + height {
                self.gridmap.set_bit_value((x, y), self.sample_grid[x][y].state != 0.0);
            }
        }
    }

    /// Initializes the gridmap from the sampling grid
    pub fn init_gridmap_radius(&mut self, (x, y): (usize, usize), radius: usize) {
        let radius = radius + 1;
        let x_min = x.saturating_sub(radius);
        let y_min = y.saturating_sub(radius);
        let x_max = (x + radius).min(self.width);
        let y_max = (y + radius).min(self.height);
        self.init_gridmap_area((x_min, y_min), x_max - x_min, y_max - y_min);
    }

    /// Initializes the gridmap from the sampling grid
    pub fn init_gridmap(&mut self) {
        self.init_gridmap_area((0, 0), self.width, self.height);
    }

    /// Initializes the ground truth grid from the sampling grid
    pub fn init_ground_truth(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.ground_truth.set_bit_value((x, y), self.sample_grid[x][y].state != 0.0);
            }
        }
    }

    /// Blurs the sampling grid with a gaussian kernal.
    /// Note that this operation sets all covariances to 1.0
    pub fn blur_samplegrid(&mut self, size: usize, sigma: f32) {
        let kernal = gaussian_kernal(size, sigma);
        self.sample_grid = convolve2d(
            &self.sample_grid,
            &kernal,
            ConvResolve::Nearest,
        );
    }

    /// Samples a cell with a given chance
    pub fn sample(&mut self, (x, y): (usize, usize)) {
        let value = self.sample_grid[x][y].state != 0.0 && rand::random::<f32>() < self.sample_grid[x][y].state;
        self.gridmap.set_bit_value((x, y), value);
    }

    /// Samples an area of the grid
    pub fn sample_area(&mut self, (x, y): (usize, usize), width: usize, height: usize) {
        for x in x..x + width {
            for y in y..y + height {
                self.sample((x, y));
            }
        }
    }

    // Calculates the radius of the sampling area
    pub fn radius_calc(&self, (x, y): (usize, usize), radius: usize) -> ((usize, usize), usize, usize) {
        let radius = radius + 1;
        let x_min = x.saturating_sub(radius);
        let y_min = y.saturating_sub(radius);
        let x_max = (x + radius).min(self.width);
        let y_max = (y + radius).min(self.height);
        ((x_min, y_min), x_max - x_min, y_max - y_min)
    }

    /// Samples a cell with a given chance
    pub fn sample_radius(&mut self, (x, y): (usize, usize), radius: usize) {
        let (n, width, height) = self.radius_calc((x, y), radius);
        self.sample_area(n, width, height);
    }

    /// Samples all cells in the grid
    pub fn sample_all(&mut self) {
        self.sample_area((0, 0), self.width, self.height)
    }

    /// Samples a cell with a given chance
    /// ## Arguments
    /// * `(x, y)` - The coordinate of the cell
    /// * `measurement_covariance` - The variance of the measurement where 0.0 is a perfect measurement
    pub fn update_node(&mut self, (x, y): (usize, usize), measurement_covariance: f32) {
        let measurement = self.ground_truth.get_bit_value((x, y)) as u8 as f32;
        self.sample_grid[x][y].update(measurement, measurement_covariance);
    }

    /// Updates the kernal based upon a gaussian kernal with a radius.
    pub fn update_node_kern(&mut self, (x, y): (usize, usize), radius: usize) {
        let kernel = gaussian_kernal(radius, 1.0);
        let kernel_size = (kernel.len(), kernel.len());
        for (n, (i, j)) in matrix_overlay((self.width, self.height), kernel_size, (x, y)) {
            self.update_node(n, kernel[i][j]);
        }
    }

    /// Checks if within bounds
    fn bound_check(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn print_sampling_cells(&self, path: Option<Vec<(usize, usize)>>) -> String {
        print_cells(self.width, self.height, |x, y| self.sample_grid[x][y].state != 0.0, path)
    }

    pub fn plot_sampling_cells(&self, output_file: &str, path: Option<Vec<(usize, usize)>>, heatmap: Option<Vec<((usize, usize), f64)>>) {
        plot_cells(self.width, self.height, output_file, |x, y| self.sample_grid[x][y].state != 0.0, path, heatmap)
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
        let grid = SampleGrid::new_from_string(map_str.to_string());
        assert_eq!(grid.ground_truth.print_cells(None), map_str);
        assert_eq!(grid.sample_grid[0][0].state, 1.0);
        assert_eq!(grid.sample_grid[1][0].state, 1.0);
        assert_eq!(grid.sample_grid[0][1].state, 0.0);
        assert_eq!(grid.gridmap.get_bit_value((0, 0)), true);
        assert_eq!(grid.gridmap.get_bit_value((1, 0)), true);
        assert_eq!(grid.gridmap.get_bit_value((0, 1)), false);
    }

    #[test]
    fn test_gridmap_init() {
        let mut grid = SampleGrid::new_from_string("@....\n.....\n.....\n.....\n".to_string());
        grid.init_gridmap_radius((0, 0), 2);
        grid.init_ground_truth();
        assert_eq!(grid.ground_truth.print_cells(None), "@....\n.....\n.....\n.....\n");
        assert_eq!(grid.gridmap.print_cells(None), "@..@@\n...@@\n...@@\n@@@@@\n");
    }

    #[test]
    fn test_blur() {
        let mut grid = SampleGrid::new_from_string("@....\n@@...\n@@@..\n@@@..\n@@...\n".to_string());
        grid.blur_samplegrid(3, 1.0);
        let grid_sample: Vec<Vec<f32>> = grid.sample_grid
            .iter()
            .map(|row| row.iter().map(|node| node.state).collect())
            .collect();
        assert_eq!(grid_sample, vec![
            vec![0.19895503, 0.07511361, 0.0, 0.0, 0.0],
            vec![0.60209, 0.32279643, 0.07511361, 0.07511361, 0.19895503],
            vec![0.9248864, 0.6772036, 0.39791006, 0.39791006, 0.60209],
            vec![1.0, 0.9248864, 0.801045, 0.801045, 0.9248864],
            vec![1.0, 1.0, 1.0, 1.0, 1.0]
        ]);
    }
}
