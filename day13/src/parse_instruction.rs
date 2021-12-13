use std::collections::HashSet;

use crate::parse_input::FoldLocation;

pub fn parse_instruction(
    grid: &HashSet<(u32, u32)>,
    instruction: &FoldLocation,
) -> HashSet<(u32, u32)> {
    let mut new_grid = HashSet::new();

    for &(x, y) in grid.iter() {
        match instruction {
            FoldLocation::X(fold_location) if &x > fold_location => {
                new_grid.insert((2 * fold_location - x, y));
            }
            FoldLocation::Y(fold_location) if &y > fold_location => {
                new_grid.insert((x, 2 * fold_location - y));
            }
            _ => {
                new_grid.insert((x, y));
            }
        }
    }

    new_grid
}
