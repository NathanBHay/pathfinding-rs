use crate::domains::bitpackedgrid::BitPackedGrid;
use crate::domains::samplinggrid_depr::SamplingGrid;
use crate::fov::fieldofvision::{raycasting_with_dist, raycasting};
use crate::fov::linedrawing::bresenham;
use crate::heuristics::distance::manhattan_distance;

impl BitPackedGrid {

    /// Returns an iterator over the k neighbors of a cell.
    pub fn kexpand(&self, (x, y): (usize, usize), k: usize) -> impl Iterator<Item = ((usize, usize), usize)> {
        let mut neighbors = Vec::new();
        for i in x.saturating_sub(k)..=x + k {
            for j in y.saturating_sub(k)..=y + k {
                if self.get_bit_value(i, j) && !(i == x && j == y) {
                    neighbors.push(((i, j), (i as isize - x as isize).abs() as usize + (j as isize - y as isize).abs() as usize));
                }
            }
        }
        neighbors.into_iter()
    }

    /// Find the neighbors of a cell that are visible from that cell.
    pub fn raycast_expand(&self, (x, y): (usize, usize), radius: usize) -> Vec<((usize, usize), usize)> {
        let mut visible = raycasting_with_dist((x, y), radius, |x, y| self.get_bit_value(x, y), |x| x * x );
        visible.retain(|(p, _)| p != &(x, y));
        visible
    }
}

impl SamplingGrid {
    pub fn raycast_expand(&self, (x, y): (usize, usize), radius: usize) -> Vec<((usize, usize), usize)> {
        raycasting_with_dist((x, y), radius, |x, y| self.get_sample_grid_value(x, y) > 0, |x| x)
            .iter()
            .filter(|(p, c)| p != &(x, y))
            .map(|((x, y), c)| ((*x, *y), self.get_sample_grid_value(*x, *y) * c))
            .collect::<Vec<_>>()
    }

    pub fn raycast_sample(&mut self, (x, y): (usize, usize), radius: usize) {
        raycasting(
            (x, y), 
            radius, 
            |x1, y1| self.sample_with_chance(x, y, 1.0 - (0.25 * (manhattan_distance((x, y), (x1, y1)).saturating_sub(1)) as f32))
        );
    }
}

#[cfg(test)]
mod tests {

    use super::BitPackedGrid;

    #[test]
    fn test_bitpackedgrid_new() {
        let grid = BitPackedGrid::create_from_string(
            "@....\n.....\n.@...\n....@\n.@...\n".to_string()
        );
        assert_eq!(grid.kexpand((2, 2), 2).count(), 20);
    }

    #[test]
    fn test_bitpackedgrid_raycast_expand() {
        let grid = BitPackedGrid::create_from_string(
            "@....\n.....\n.@...\n..@.@\n.@...\n".to_string()
        );
        println!("{:?}", grid.raycast_expand((2, 2), 2));
        // assert_eq!(grid.raycast_expand((2, 2), 2).len(), 20);
    }
}