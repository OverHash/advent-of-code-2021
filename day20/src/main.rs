use std::fs;

mod enhance;
mod parse_input;
mod part1;
mod part2;
mod updated_tile;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1::solve(&input));
    println!("{}", part2::solve(&input));
}
