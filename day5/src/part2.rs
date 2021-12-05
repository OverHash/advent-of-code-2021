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

        if start_x == finish_x || start_y == finish_y {
            for x in start_x.min(finish_x)..=finish_x.max(start_x) {
                for y in start_y.min(finish_y)..=finish_y.max(start_y) {
                    *lines.entry((x, y)).or_default() += 1;
                }
            }
        } else {
            let mut current_x = start_x;
            let mut current_y = start_y;

            loop {
                *lines.entry((current_x, current_y)).or_default() += 1;

                // check to see if we have reached the destination
                if current_x == finish_x {
                    break;
                }

                if current_x > finish_x {
                    current_x -= 1;
                } else {
                    current_x += 1;
                }
                if current_y > finish_y {
                    current_y -= 1;
                } else {
                    current_y += 1;
                }
            }
        }
    }

    lines.iter().filter(|(_, &d)| d > 1).count() as u32
}
