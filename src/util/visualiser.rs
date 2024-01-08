use serde_json::json;
use std::{fs::File, collections::HashMap};

use crate::domains::{samplegrid::SampleGrid, bitpackedgrid::BitPackedGrid, adjacencylist::AdjacencyList};
use crate::domains::neighbors;
pub struct Visualiser {
    file_path: String,
    start: Option<(usize, usize)>,
    goal: Option<(usize, usize)>,
}


impl Visualiser {

    pub fn new(file_path: &str, sample_grid: &SampleGrid, start: Option<(usize, usize)>, goal: Option<(usize, usize)>) -> Self {
        let visualiser = Visualiser {
            file_path: file_path.to_string(),
            start,
            goal,
        };
        visualiser.visualise_ground_truth(&sample_grid.ground_truth);
        visualiser
    }

    fn visualise_ground_truth(&self, grid: &BitPackedGrid) {
        let mut ground_truth = vec![vec![false; grid.original_height]; grid.original_width];
        for x in 0..grid.original_width {
            for y in 0..grid.original_height {
                ground_truth[x][y] = grid.get_bit_value((x, y));
            }
        }
        let ground_truth = json!({
            "width": grid.original_width,
            "height": grid.original_height,
            "grid": ground_truth,
            "start": self.start,
            "goal": self.goal,
        });
        let mut file = File::create(format!("{}_ground_truth.json", self.file_path)).unwrap();
        serde_json::to_writer_pretty(&mut file, &ground_truth).unwrap();
    }

    pub fn visualise_iteration(&self, sample_grid: &SampleGrid, iteration: usize, current: Option<(usize, usize)>, next: Option<(usize, usize)>, paths: &HashMap<(usize, usize), usize>) {
        let sample_grid = json!({
            "sample_grid": get_sample_grid(&sample_grid),
            "start": current,
            "goal": self.goal,
            "next": next,
            "paths": hashmap_to_adjlist(paths).iter().collect::<Vec<_>>(),
        });
        let mut file = File::create(format!("{}_step_{}.json", self.file_path, iteration)).unwrap();
        serde_json::to_writer_pretty(&mut file, &sample_grid).unwrap();
    }

    pub fn visualise_final_path(&self, final_path: &Vec<(usize, usize)>) {
        let mut paths = HashMap::new();
        for node in final_path {
            *paths.entry(*node).or_insert(0) += 1;
        }
        let sample_grid = json!({
            "start": self.start,
            "goal": self.goal,
            "paths": hashmap_to_adjlist(&paths).iter().collect::<Vec<_>>(),
        });
        let mut file = File::create(format!("{}_final_path.json", self.file_path)).unwrap();
        serde_json::to_writer_pretty(&mut file, &sample_grid).unwrap();
    }
}

fn get_sample_grid(grid: &SampleGrid) -> Vec<Vec<f32>> {
    let mut sample_grid = vec![vec![0.0; grid.height]; grid.width];
    for x in 0..grid.width {
        for y in 0..grid.height {
            sample_grid[x][y] = grid.sample_grid[x][y].state;
        }
    }
    sample_grid
}

fn hashmap_to_adjlist(map: &HashMap<(usize, usize), usize>) -> AdjacencyList<(usize, usize), f32> {
    let mut adjlist = AdjacencyList::new();
    let mut max = 0;
    for ((x, y), w) in map.iter() {
        adjlist.add_node((*x, *y));
        for (dest_x, dest_y) in neighbors(*x, *y, false) {
            if let Some(dest_w) = map.get(&(dest_x, dest_y)) {
                adjlist.add_edge((*x, *y), (dest_x, dest_y), *w.min(dest_w) as f32);
                if *w > max {
                    max = *w;
                }
            }
        }
    }
    for node in adjlist.iter_mut() {
        for (_, w) in node.iter_mut() {
            *w = *w / max as f32;
        }
    }
    adjlist
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_visualise() {
        let mut grid = SampleGrid::new_from_string(".....\n....@\n.....\n...@.\n...@.\n".to_string());
        grid.blur_samplegrid(2, 1.0);
        let visualiser = Visualiser::new("test", &grid, Some((0, 0)), Some((4, 4)));
        let mut paths = HashMap::new();
        paths.insert((0, 0), 2);
        paths.insert((1, 0), 2);
        paths.insert((2, 0), 1);
        paths.insert((1, 1), 1);
        visualiser.visualise_iteration(&grid, 0, Some((0, 0)), Some((1,0)), &paths);

    }
}