use std::collections::HashMap;

use crate::parse_input::{parse_input, ParsedInput};

pub fn solve(input: &str) -> u32 {
    let input = parse_input(input);

    let mut lines: HashMap<(u32, u32), u32> = HashMap::new();
    for line in input {
        let ParsedInput {
            start_x,
            start_y,
            finish_x,
            finish_y,
        } = line;

        // only consider lines in which x1 = x2 or y1 = y2
        if !(start_x == finish_x || start_y == finish_y) {
            continue;
        }

        for x in start_x.min(finish_x)..=start_x.max(finish_x) {
            for y in start_y.min(finish_y)..=start_y.max(finish_y) {
                *lines.entry((x, y)).or_default() += 1;
            }
        }
    }

    lines.iter().filter(|(_, &d)| d > 1).count() as u32
}
