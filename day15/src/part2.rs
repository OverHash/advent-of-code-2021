use crate::find_shortest_path::find_shortest_path;

pub fn solve(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let expanded_grid = (0..(5 * grid.len()))
        .map(|x| {
            (0..(5 * grid[0].len()))
                .map(|y| {
                    let x_level = (x / grid.len()) as u32;
                    let y_level = (y / grid[0].len()) as u32;
                    let cost = grid[x % grid.len()][y % grid[0].len()] + x_level + y_level;
                    if cost < 10 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    find_shortest_path(&expanded_grid)
}
