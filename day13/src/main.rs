use std::fs;

mod parse_input;
mod parse_instruction;
mod part1;
mod part2;

fn main() {
    let input = parse_input::parse_input(&fs::read_to_string("input.txt").unwrap());
    println!("{}", part1::solve(&input));
    println!("{}", part2::solve(&input));
}
