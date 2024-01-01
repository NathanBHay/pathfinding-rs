//! # Bit-Packed Grid Maps
//! A grid map representation that uses bitpacking to store the map.
//! This is the fastest grid map implementation, as it uses bit manipulations
//! to change the map while also being the smallest as it compresses the map
//! into a bitpacked array. This array is stored as an array of numbers, where 
//! each binary digit in the number represents a cell in the map. These cells
//! are additionally padded with 2 rows and columns to avoid travelling out of 
//! bounds.
//! 
//! This implementation is based upon Warthog's implementation of bitpacked grid
//! maps, which can be found: https://bitbucket.org/dharabor/pathfinding/

use std::vec;

use super::{create_map_from_string, print_cells, neighbors, plot_cells};

/// A grid of bits packed into usize-bit words
#[derive(Debug)]
pub struct BitPackedGrid {
    pub original_height: usize,
    pub original_width: usize,
    map_height: usize,
    map_width: usize,
    map_width_in_words: usize,
    map_size: usize,
    map_cells: Box<[usize]>,
}

impl BitPackedGrid {
    const PADDING: usize = 2;
    const BITS_PER_WORD: usize = usize::BITS as usize;
    const LOG2_BITS_PER_WORD: usize = usize::BITS.trailing_zeros() as usize;
    const INDEX_MASK: usize = BitPackedGrid::BITS_PER_WORD - 1;

    /// Create a new BitPackedGrid
    pub fn new(width: usize, height: usize) -> BitPackedGrid {
        let map_width_in_words = (width >> BitPackedGrid::LOG2_BITS_PER_WORD) + 1;
        let map_width = map_width_in_words << BitPackedGrid::LOG2_BITS_PER_WORD;
        let map_height = height + 2 * BitPackedGrid::PADDING;
        let map_size = (map_width * map_height) >> BitPackedGrid::LOG2_BITS_PER_WORD;
        let map_cells: Box<[usize]> = vec![0; map_size as usize].into_boxed_slice();
        BitPackedGrid {
            original_height: height,
            original_width: width,
            map_height,
            map_width,
            map_width_in_words,
            map_size,
            map_cells,
        }
    }

    /// Create a bitpacked grid from a string
    /// ## Arguments
    /// * `map` - A string representing the map where . is a free cell
    pub fn new_from_string(map: String) -> BitPackedGrid {
        create_map_from_string(map, BitPackedGrid::new, |grid, x, y| {
            grid.set_bit_value(x, y, true)  
        })
    }

    /// Create a bitpacked grid from a file
    /// ## Arguments
    /// * `filename` - The name of the file to read from
    /// ## Returns
    /// A bitpacked grid created from a file
    pub fn new_from_file(filename: &str) -> BitPackedGrid {
        let s = std::fs::read_to_string(filename).expect("Unable to read file");
        BitPackedGrid::new_from_string(s)
    }

    /// Set the value if a but at a given x, y coordinate to be true or false
    pub fn set_bit_value(&mut self, x: usize, y: usize, value: bool) {
        let map_id = self.get_map_id(x, y);
        let word_index = map_id >> BitPackedGrid::LOG2_BITS_PER_WORD;
        let mask = 1 << (map_id & BitPackedGrid::INDEX_MASK);
        if value {
            self.map_cells[word_index as usize] |= mask;
        } else {
            self.map_cells[word_index as usize] &= !mask;
        }
    }

    /// Get the value of a bit at a given x, y coordinate
    pub fn get_bit_value(&self, x: usize, y: usize) -> bool {
        let map_id = self.get_map_id(x, y);
        let word_index = map_id >> BitPackedGrid::LOG2_BITS_PER_WORD;
        let mask = 1 << (map_id & BitPackedGrid::INDEX_MASK);
        (self.map_cells[word_index as usize] & mask) != 0
    }

    /// Check if a given x, y coordinate is within the bounds of the map
    pub fn bounds_check(&self, x: usize, y: usize) -> bool {
        x < self.original_width && y < self.original_height
    }

    /// Convert x, y to map id
    /// ## Arguments
    /// * `x` - The x coordinate of the cell [0, original width)
    /// * `y` - The y coordinate of the cell [0, original height)
    /// ## Returns
    /// The map id of the cell
    fn get_map_id(&self, x: usize, y: usize) -> usize {
        self.map_width * (y.wrapping_add(BitPackedGrid::PADDING))
            + (x.wrapping_add(BitPackedGrid::PADDING))
    }

    /// Convert map id to x, y
    fn map_id_to_xy(&self, map_id: usize) -> (usize, usize) {
        let y = map_id / self.original_height;
        let x = map_id - y * self.original_width;
        (x, y)
    }

    /// Prints the grid map where . is a free cell and @ is an obstacle
    pub fn print_cells(&self) -> String {
        print_cells(self.original_width, self.original_height, |x, y| {
            self.get_bit_value(x, y)
        }, None)
    }

    /// Get the neighbors of a given cell
    pub fn adjacent(&self, x: usize, y: usize, diagonal: bool) -> impl Iterator<Item = (usize, usize)> + '_ {
        neighbors(x, y, diagonal).filter(move |(x, y)| self.get_bit_value(*x, *y))
    }

    pub fn grid_section(&self, x: usize, y: usize, width: usize, height: usize) -> Vec<Vec<f32>> {
        let mut grid = vec![vec![0.0; width]; height];
        for i in x..(x+width) {
            for j in y..(y+height) {
                grid[i-x][j-y] = self.get_bit_value(i, j) as u8 as f32;
            }
        }
        grid
    }

    pub fn plot_cells(&self, filename: &str, path: Option<Vec<(usize, usize)>>, heatmap: Option<Vec<((usize, usize), f64)>>) {
        plot_cells(self.original_width, self.original_height, filename, |x, y| {
            self.get_bit_value(x, y)
        }, path, heatmap)
    }
}

#[cfg(test)]
mod tests {
    use super::BitPackedGrid;

    #[test]
    fn test_bitpackedgrid_new() {
        let grid = BitPackedGrid::new(128, 128);
        assert_eq!(grid.original_height, 128);
        assert_eq!(grid.original_width, 128);
        assert_eq!(grid.map_height, 132);
        assert_eq!(grid.map_width, 192);
        assert_eq!(grid.map_width_in_words, 3);
        assert_eq!(grid.map_size, 396);
        assert_eq!(grid.map_cells.len(), 396);
    }

    #[test]
    fn test_bitpackedgrid_set_bit_value() {
        let mut grid = BitPackedGrid::new(16, 16);
        grid.set_bit_value(0, 0, true);
        assert_eq!(grid.get_bit_value(0, 0), true);
        grid.set_bit_value(0, 0, false);
        assert_eq!(grid.get_bit_value(0, 0), false);
        grid.set_bit_value(15, 15, true);
        assert_eq!(grid.get_bit_value(15, 15), true);
        grid.set_bit_value(15, 5, true);
        assert_eq!(grid.get_bit_value(15, 5), true);
    }

    #[test]
    fn test_bitpackedgrid_create() {
        let map_str = ".....\n.@.@.\n.@.@.\n.@.@.\n.....\n....@\n";
        let grid = BitPackedGrid::new_from_string(map_str.to_string());
        assert_eq!(grid.print_cells(), map_str);
    }

    #[test]
    fn test_bitpackedgrid_get_neighbours() {
        let grid = BitPackedGrid::new_from_string(".....\n.@.@.\n.@.@.\n.@.@.\n.....\n....@\n".to_string());
        assert_eq!(grid.adjacent(0, 0, false).collect::<Vec<_>>(), vec![(1, 0), (0, 1)]);
    }
}
