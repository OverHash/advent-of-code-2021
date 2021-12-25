use std::fs;

mod part1;

mod parse_input;
mod rotation;
mod solve;
mod step;

pub use parse_input::parse_input;
pub use rotation::Rotation;
pub use solve::solve as solve_solution;
pub use step::step;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1::solve(&input));
}
