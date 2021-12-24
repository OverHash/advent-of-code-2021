use std::fs;

mod find_scan_merge;
mod get_rotation;
mod merge_scan;
mod parse_input;
mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1::solve(&input));
    println!("{}", part2::solve(&input));
}
