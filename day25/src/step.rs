use itertools::Itertools;

use crate::Rotation;

pub fn step(
    grid: Vec<Vec<Rotation>>,
    right_step: usize,
    down_step: usize,
) -> (Vec<Vec<Rotation>>, bool) {
    let (height, width) = (grid.len(), grid[0].len());

    let mut new_grid = vec![vec![Rotation::None; width]; height];

    let mut moved = false;
    for (y, x) in (0..height).cartesian_product(0..width) {
        match grid[y][x] {
            Rotation::Right if grid[y][(x + down_step) % width] == Rotation::None => {
                new_grid[y][(x + down_step) % width] = Rotation::Right;
                moved = true;
            }
            Rotation::Down if grid[(y + right_step) % height][x] == Rotation::None => {
                new_grid[(y + right_step) % height][x] = Rotation::Down;
                moved = true;
            }
            Rotation::Right => new_grid[y][x] = Rotation::Right,
            Rotation::Down => new_grid[y][x] = Rotation::Down,
            _ => {}
        }
    }

    (new_grid, moved)
}
