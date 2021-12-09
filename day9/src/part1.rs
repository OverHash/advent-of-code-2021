/// Retrieves the height of a neighbor node by doing additional checks on the indexes first.
/// Returns `9` if the neighbor is out of bounds.
fn get_neighbor_height(grid: &[Vec<u32>], x: usize, y: usize) -> u32 {
    // check overflow bounds
    if y >= grid.len() || x >= grid[y].len() {
        return 9;
    }

    grid[y][x]
}

pub fn solve(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut risk = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            // get heights of elements greater than current position
            let mut neighbor_heights = vec![
                get_neighbor_height(&grid, x + 1, y),
                get_neighbor_height(&grid, x, y + 1),
            ];

            // check for underflow calculating height
            if x != 0 {
                neighbor_heights.push(get_neighbor_height(&grid, x - 1, y));
            }
            if y != 0 {
                neighbor_heights.push(get_neighbor_height(&grid, x, y - 1));
            }

            if neighbor_heights.iter().min().unwrap() > height {
                risk += 1 + height;
            }
        }
    }

    risk
}
