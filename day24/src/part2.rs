use crate::{parse_input, solve_solution};

pub fn solve(input: &str) -> i64 {
    let input = parse_input(input);

    solve_solution(input, true)
}
