struct SamplingGrid {
    pub grid: Vec<Vec<f32>>,
}

impl SamplingGrid {

    /// Creates a new sampling grid
    pub fn new() -> SamplingGrid {
        SamplingGrid {
            grid: vec![vec![0.0; 0]; 0],
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
            grid: vec![vec![0.0; height]; width],
        }
    }

}