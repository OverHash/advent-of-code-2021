use crate::{enhance::enhance, parse_input::parse_input};
pub fn solve(input: &str) -> u32 {
    let (enhancement, mut input) = parse_input(input);

    for i in 0..50 {
        input = enhance(&input, &enhancement, i & 1);
    }

    input.iter().flatten().filter(|&&b| b == 1).count() as u32
}
