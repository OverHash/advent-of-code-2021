use std::collections::HashSet;

use crate::{parse_input::ParsedInput, parse_instruction::parse_instruction};

pub fn solve(input: &ParsedInput) -> String {
    let mut grid = HashSet::new();
    for coordinate in &input.coordinates {
        grid.insert((coordinate.x, coordinate.y));
    }

    for fold in &input.fold_coordinates {
        grid = parse_instruction(&grid, fold);
    }

    let max_x = grid.iter().map(|n| n.0).max().unwrap();
    let max_y = grid.iter().map(|n| n.1).max().unwrap();

    let mut display_grid = String::new();
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            display_grid.push(match grid.contains(&(x, y)) {
                true => '█',
                false => ' ',
            });
        }
        display_grid.push('\n');
    }

    display_grid
}
