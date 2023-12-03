//! Distance heuristics used to compute the distance between two points in a grid.
//! These heuristics are used on grid maps in pathfinding algorithms such as A*.

pub fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn euclidean_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    (((x1 as f32 - x2 as f32).powi(2) + (y1 as f32 - y2 as f32).powi(2)) as f32).sqrt()
}

pub fn chebyshev_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs().max((y1 - y2).abs())
}

pub fn octile_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    let min = dx.min(dy);
    let max = dx.max(dy);
    (min as f32 * 1.4142135623730951) + (max - min) as f32
}