use std::vec;

pub struct BitPackedGrid {
    original_height: u32,
    original_width: u32,
    map_height: u32,
    map_width: u32,
    map_width_in_words: u32,
    map_size: u32,
    map_cells: Box<[u32]>,
}

impl BitPackedGrid {
    const PADDING: u32 = 2;
    const BITS_PER_WORD: u32 = 32;  
    const LOG2_BITS_PER_WORD: u32 = 5;
    const INDEX_MASK: u32 = BitPackedGrid::BITS_PER_WORD - 1;

    pub fn new(height: u32, width: u32) -> BitPackedGrid {
        let map_width_in_words = (width >> BitPackedGrid::LOG2_BITS_PER_WORD) + 1;
        let map_width = map_width_in_words << BitPackedGrid::LOG2_BITS_PER_WORD;
        let map_height = height + 2*BitPackedGrid::PADDING;
        let map_size = (map_width * map_height) >> BitPackedGrid::LOG2_BITS_PER_WORD;
        let map_cells: Box<[u32]> = vec![0; map_size as usize].into_boxed_slice();
        BitPackedGrid {
            original_height: height,
            original_width: width,
            map_height,
            map_width,
            map_width_in_words,
            map_size,
            map_cells
        }
    }

    pub fn create(map: String) -> BitPackedGrid {
        let map = map.trim();
        let height = u32::try_from(map.lines().count()).unwrap();
        let width = u32::try_from(map.lines().next().map(|x| x.len()).unwrap()).unwrap();
        let mut grid = BitPackedGrid::new(
            u32::try_from(height).unwrap(), 
            u32::try_from(width).unwrap()
        );
        for (i, line) in map.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '@' {
                    grid.set_bit_value(
                        u32::try_from(j).unwrap(), 
                        u32::try_from(i).unwrap(), 
                        true
                    );
                }
            }
        }
        grid
    }

    fn set_bit_value(&mut self, x: u32, y: u32, value: bool) {
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
    fn get_bit_value(&self, x: u32, y: u32) -> bool {
        let map_id = self.get_map_id(x, y);
        let word_index = map_id >> BitPackedGrid::LOG2_BITS_PER_WORD;
        let mask = 1 << (map_id & BitPackedGrid::INDEX_MASK);
        (self.map_cells[word_index as usize] & mask) != 0
    }

    /// Convert x, y to map id
    fn get_map_id(&self, x: u32, y: u32) -> u32 {
        self.map_width * (y + BitPackedGrid::PADDING) + (x + BitPackedGrid::PADDING)
    }

    /// Convert map id to x, y
    fn map_id_to_xy(&self, map_id: u32) -> (u32, u32) {
        let y = map_id / self.original_height;
        let x = map_id - y * self.original_width;
        (x, y)
    }
}