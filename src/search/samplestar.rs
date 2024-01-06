//! # SampleStar
//! A 

use std::collections::HashMap;

use crate::{domains::samplegrid::SampleGrid, heuristics::distance::manhattan_distance, util::visualiser::{Visualiser, self}};

use super::astar::astar;

/// Sample Star Algorithm
/// ## Arguments
/// * `grid` - The sampling grid to search on.
/// * `start` - The starting node.
/// * `goal` - The goal node.
/// * `epoch` - The number of times to sample each node.
/// * `radius` - The radius to sample around each node.
pub struct SampleStar {
    grid: SampleGrid,
    current: (usize, usize),
    goal: (usize, usize),
    epoch: usize,
    radius: usize,
    final_path: Vec<(usize, usize)>,
    visualiser: Option<Visualiser>,
}

impl SampleStar {

    /// Creates a new SampleStar algorithm
    pub fn new(grid: SampleGrid, start: (usize, usize), goal: (usize, usize), epoch:usize, radius: usize) -> Self {
        Self {
            grid,
            current: start,
            goal,
            epoch,
            radius,
            final_path: Vec::new(),
            visualiser: None,
        }
    }

    /// Run the algorithm for one step
    pub fn step(&mut self) -> bool {
        if self.current == self.goal {
            return true;
        }
        let mut heatmap = HashMap::new();
        // let mut cached_paths = HashMap::new();
        self.grid.update_node_kern(self.current, self.radius);
        self.grid.init_gridmap_radius(self.current, self.radius + 1); // +1 to account for previous update
        for _ in 0..self.epoch {
            self.grid.sample_radius(self.current, self.radius);
            let path = astar(
                |n| self.grid.gridmap.adjacent1(*n),
                self.current,
                |n| *n == self.goal,
                |n| manhattan_distance(*n, self.goal),
            );
            if let Some((path, _)) = path {
                for node in path {
                    let count = heatmap.entry(node).or_insert(1);
                    *count += 1;
                }
            }
        }
        let old = self.current;

        self.current = self.grid.gridmap.adjacent(self.current, false)
            .max_by_key(|n| heatmap.get(n))
            .unwrap_or(self.current) // If no path exists stay at node
            .clone();
        self.final_path.push(self.current);

        if let Some(visualiser) = &self.visualiser {
            visualiser.visualise_iteration(&self.grid, self.final_path.len(), Some(old), Some(self.current), &heatmap);
        }
        false
    }

    pub fn add_visualiser(&mut self, file_path: &str) {
        self.visualiser = Some(Visualiser::new(file_path, &self.grid, Some(self.current), Some(self.goal)));
    }
}

/*
Heuristics could include ones that take into account probability of being an obstacle:
`self.grid.sample_grid[x][y].state * manhattan_distance(n*, self.goal)`
*/

#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn test_samplestar() {
        let file = "tests/basic.map";
        let start = (1, 1);
        let goal = (30, 30);
        // let file = "tests/map.map";
        // let start = (225, 225);
        // let goal = (70, 40);
        let mut grid = SampleGrid::new_from_file(file);
        grid.blur_samplegrid(5, 1.0);
        let mut samplestar = SampleStar::new(grid, start, goal, 1, 2);
        samplestar.add_visualiser("test");

        // while !samplestar.step() {}
        for _ in 0..100 {
            samplestar.step();
        }
        samplestar.visualiser.as_ref().unwrap().visualise_final_path(&samplestar.final_path);
        assert!(false);
        // assert_eq!(samplestar.final_path.len(), 100);
    }
}