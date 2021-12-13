use std::collections::HashSet;

use crate::{parse_input::ParsedInput, parse_instruction::parse_instruction};

pub fn solve(input: &ParsedInput) -> u32 {
    let mut grid = HashSet::new();
    for coordinate in &input.coordinates {
        grid.insert((coordinate.x, coordinate.y));
    }

    let fold = &input.fold_coordinates[0];
    parse_instruction(&grid, fold).len() as u32
}
