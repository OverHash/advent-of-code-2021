use crate::find_shortest_path::find_shortest_path;

pub fn solve(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    find_shortest_path(&grid)
}
