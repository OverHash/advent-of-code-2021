use std::collections::HashSet;

fn find_basin_size(
    grid: &[Vec<u32>],
    scanned: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) -> u32 {
    // check to see we have not already searched this tile
    if scanned.contains(&(x, y)) {
        return 0;
    }
    // get the tile
    let row = grid.get(y);
    if row.is_none() {
        return 0;
    }

    let height = row.unwrap().get(x);
    if height.is_none() || height.unwrap() == &9 {
        return 0;
    }

    scanned.insert((x, y));

    let mut size = 1;
    size += find_basin_size(grid, scanned, x + 1, y);
    size += find_basin_size(grid, scanned, x, y + 1);

    // since we have indexes as usize, we must check that `x-1` and `y-1` is > 0 before we subtract
    if x > 0 {
        size += find_basin_size(grid, scanned, x - 1, y);
    }
    if y > 0 {
        size += find_basin_size(grid, scanned, x, y - 1);
    }

    size
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

    let mut points_scanned = HashSet::new();

    let mut basins = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for x in 0..=row.len() {
            if points_scanned.contains(&(x, y)) {
                continue;
            }

            // this is a new basin, add the height
            basins.push(find_basin_size(&grid, &mut points_scanned, x, y));
        }
    }

    basins.sort_unstable_by(|a, b| b.cmp(a));

    (basins[0] * basins[1] * basins[2]) as u32
}
