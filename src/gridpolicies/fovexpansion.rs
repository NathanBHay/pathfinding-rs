//! Expansion policies that use field of vision to determine which cells to expand into.

use crate::domains::bitpackedgrid::BitPackedGrid;
use crate::fov::fieldofvision::raycasting_with_dist;

impl BitPackedGrid {
    /// Find the neighbors of a cell that are visible from that cell.
    pub fn raycast_expand(&self, (x, y): (usize, usize), radius: usize) -> Vec<((usize, usize), usize)> {
        let mut visible = raycasting_with_dist((x, y), radius, |x, y| self.get_bit_value((x, y)), |x| x * x );
        visible.retain(|(p, _)| p != &(x, y));
        visible
    }
}

// TODO: Reimplement visibility for sample grid where raycast outputs a kernel of:
// [1, 1, 1]
// [0, 1, 1]
// [1, 0, 0]
// Where 1 implies visible and 0 implies not visible. Therefore through 
// elementwise multiplication with the gaussian kernel we can find the 
// we can find if vis[i][j] then kernel[i][j] else 1.0

// May still be useful
// impl SamplingGrid {
    // pub fn raycast_expand(&self, (x, y): (usize, usize), radius: usize) -> Vec<((usize, usize), usize)> {
        // raycasting_with_dist((x, y), radius, |x, y| self.get_sample_grid_value(x, y) > 0, |x| x)
            // .iter()
            // .filter(|(p, c)| p != &(x, y))
            // .map(|((x, y), c)| ((*x, *y), self.get_sample_grid_value(*x, *y) * c))
            // .collect::<Vec<_>>()
    // }
// 
    // pub fn raycast_sample(&mut self, (x, y): (usize, usize), radius: usize) {
        // raycasting(
            // (x, y), 
            // radius, 
            // |x1, y1| self.sample_with_chance(x, y, 1.0 - (0.25 * (manhattan_distance((x, y), (x1, y1)).saturating_sub(1)) as f32))
        // );
    // }
// }

#[cfg(test)]
mod tests {

    use super::BitPackedGrid;

    #[test]
    fn test_bitpackedgrid_raycast_expand() {
        let grid = BitPackedGrid::new_from_string(
            "@....\n.....\n.@...\n..@.@\n.@...\n".to_string()
        );
        println!("{:?}", grid.raycast_expand((2, 2), 2));
        // assert_eq!(grid.raycast_expand((2, 2), 2).len(), 20);
    }
}