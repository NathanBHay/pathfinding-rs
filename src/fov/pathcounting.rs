use std::collections::HashMap;

/// Path counting approach to field of vision
/// Adapted from python version found at:
/// https://towardsdatascience.com/a-quick-and-clear-look-at-grid-based-visibility-bf63769fbc78
pub fn compute_visibility_from_corner(
    (x, y): (usize, usize), 
    radius: usize, 
    visibility_check: impl Fn(usize, usize) -> bool
) -> Vec<((usize, usize), f32)> {
    let mut visible = HashMap::new();
    for i in 0..=radius {
        for j in 0..=radius {
            if visibility_check(x + i, y + j) {
                visible.insert((x + i, y + j), 1.0);
            }
        }
    }
    for x in 0..=radius {
        let s = if x == 0 { 1 } else {0};
        for y in s..=radius {
            let path_count = (x as f32) * visible.get(&(x.saturating_sub(1), y)).unwrap_or(&0.0) + 
                                  (y as f32) * visible.get(&(x, y.saturating_sub(1))).unwrap_or(&0.0);
            visible.insert((x, y), path_count / ((x + y) as f32));
        }
    }
    // Grid approach kept as hashmap approach may be slower
    // let mut grid = vec![vec![0.0; radius + 1]; radius + 1];
    // for x in 0..grid[0].len() {
    //     let s = if x == 0 { 1 } else {0};
    //     for y in s..grid.len() {
    //         grid[x][y] *= ((x as f32) * grid[x-1][y] + (y as f32) * grid[x][y-1]) / ((x + y) as f32);
    //     }
    // }
    visible.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use super::compute_visibility_from_corner;

    #[test]
    fn test_path_counting() {
        let visible = compute_visibility_from_corner((10, 10), 2, |_, _| true);
        assert_eq!(visible.len(), 25);
    }
}