use std::fs;

mod is_board_solved;
mod part1;
mod part2;
mod sum_board;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1::solve(&input));
    println!("{}", part2::solve(&input));
}
