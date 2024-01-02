use std::collections::HashMap;

use crate::{domains::samplegrid::SampleGrid, heuristics::distance::manhattan_distance};

use super::astar::astar;

pub struct SampleStar {
    grid: SampleGrid,
    current: (usize, usize),
    goal: (usize, usize),
    final_path: Vec<(usize, usize)>,
    epoch: usize,
    radius: usize,
}

impl SampleStar {

    /// Creates a new SampleStar algorithm
    pub fn new(grid: SampleGrid, start: (usize, usize), goal: (usize, usize), epoch:usize, radius: usize) -> Self {
        Self {
            grid,
            current: start,
            goal,
            final_path: Vec::new(),
            epoch,
            radius,
        }
    }

    /// Run the algorithm for one step
    pub fn step(&mut self) {
        let mut heatmap = HashMap::new();
        // let mut cached_paths = HashMap::new();
        self.grid.update_node_kern(self.current, self.radius);
        for _ in 0..self.epoch {
            self.grid.sample_radius(self.current, self.radius);
            let path = astar(
                |n| self.grid.gridmap.adjacent1(*n),
                self.current,
                |n| n == &self.goal,
                |n| manhattan_distance(*n, self.goal),
            );
            if let Some((path, _)) = path {
                for node in path {
                    let count = heatmap.entry(node).or_insert(0);
                    *count += 1;
                }
            }
        }
        self.grid.init_gridmap_radius(self.current, self.radius);
        self.current = self.grid.gridmap.adjacent(self.current, false)
            .min_by_key(|n| heatmap.get(n).unwrap_or(&0))
            .unwrap()
            .clone();
        self.final_path.push(self.current);
    }
}
