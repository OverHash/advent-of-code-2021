use itertools::Itertools;

use crate::parse_number::parse_number;

pub fn solve(input: &str) -> u64 {
    let input = input.lines().map(|line| parse_number(line).unwrap().1);

    input
        .into_iter()
        .permutations(2)
        .map(|permutation| {
            permutation
                .into_iter()
                .reduce(|l, r| (l + r).reduce())
                .unwrap()
                .magnitude()
        })
        .max()
        .unwrap()
}
