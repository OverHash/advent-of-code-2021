use std::fs;

mod number;
mod parse_number;
mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1::solve(&input));
    println!("{}", part2::solve(&input));
}
