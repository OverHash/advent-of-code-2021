use crate::solve::solve as calculate_days;

pub fn solve(input: &str) -> u64 {
    calculate_days(input.split(',').flat_map(|d| d.parse()).collect(), 256)
}
