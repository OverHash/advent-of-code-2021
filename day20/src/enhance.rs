use itertools::Itertools;

use crate::updated_tile::updated_tile;

pub fn enhance(grid: &[Vec<u8>], enhancement: &[u8], val: u8) -> Vec<Vec<u8>> {
    let mut ans = vec![vec![0; grid[0].len() + 2]; grid.len() + 2];
    for (r, c) in (0..ans.len()).cartesian_product(0..ans[0].len()) {
        ans[r][c] = updated_tile(grid, enhancement, r - 1, c - 1, val);
    }
    ans
}
