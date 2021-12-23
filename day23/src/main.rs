use std::fs;

mod part1;
mod part2;

mod parse_input;
mod shortest_path;
mod state;

pub use parse_input::parse_input;
pub use shortest_path::shortest_path;
pub use state::State;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1::solve(&input));
    println!("{}", part2::solve(&input));
}
