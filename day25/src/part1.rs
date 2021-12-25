use crate::{parse_input, step};

pub fn solve(input: &str) -> i64 {
    let mut input = parse_input(input);

    for step_no in 1.. {
        let (new_map, moved) = step(input, 0, 1);
        input = new_map;

        let (new_map, moved2) = step(input, 1, 0);
        input = new_map;

        if !moved && !moved2 {
            return step_no;
        }
    }

    unreachable!()
}
