use std::{collections::HashSet, iter};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn flash(grid: &mut Vec<Vec<u32>>, seen: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> u32 {
    if seen.contains(&(x, y)) {
        return 0;
    }
    seen.insert((x, y));

    let energy = grid[y][x];

    if energy < 10 {
        return 0;
    }

    let mut flashes = 0;
    flashes += 1;

    for (x_add, y_add) in DIRECTIONS {
        let (new_x, new_y) = (x as i8 + x_add, y as i8 + y_add);
        if new_x < 0 || new_y < 0 || new_x >= WIDTH as i8 || new_y >= HEIGHT as i8 {
            continue;
        }

        let (new_x, new_y) = (new_x as usize, new_y as usize);

        grid[new_y][new_x] += 1;
        if grid[new_y][new_x] > 9 {
            flashes += flash(grid, seen, new_x, new_y);
        }
    }

    flashes
}

fn step(grid: &mut Vec<Vec<u32>>) -> u32 {
    // give one energy to every item
    (0..HEIGHT)
        .flat_map(|y| iter::repeat(y).zip(0..WIDTH))
        .for_each(|(y, x)| grid[y][x] += 1);

    // get the ones we need to flash
    let flash_grid = grid.to_owned();
    let to_flash = flash_grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter_map(move |(x, &d)| match d == 10 {
                true => Some((x, y)),
                false => None,
            })
    });

    // do the flashes
    let mut seen = HashSet::new();
    let flashes = to_flash.map(|(x, y)| flash(grid, &mut seen, x, y)).sum();

    // set all those that have flashed back to 0 if they flashed
    (0..HEIGHT)
        .flat_map(|y| iter::repeat(y).zip(0..WIDTH))
        .for_each(|(y, x)| {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
            }
        });

    flashes
}

pub fn solve(input: &str) -> u32 {
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut step_num = 0;
    loop {
        step_num += 1;
        let flashes = step(&mut grid);
        if flashes == (WIDTH as u32 * HEIGHT as u32) {
            return step_num;
        }
    }
}
