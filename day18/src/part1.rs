use crate::parse_number::parse_number;

pub fn solve(input: &str) -> u64 {
    let input = input.lines().map(|line| parse_number(line).unwrap().1);

    let sum = input.reduce(|l, r| (l + r).reduce()).unwrap();

    sum.magnitude()
}
