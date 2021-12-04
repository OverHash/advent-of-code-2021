use crate::is_board_solved::is_board_solved;
use crate::sum_board::sum_board;

pub fn solve(input: &str) -> u32 {
    let mut input = input.split("\n\n");
    let board_draw = input
        .next()
        .unwrap()
        .split(',')
        .flat_map(|d| d.parse::<u32>())
        .collect::<Vec<_>>();

    let boards = input
        .map(|board| {
            board
                .lines()
                .map(|row| row.split_whitespace().flat_map(|d| d.parse()).collect())
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<_>>();

    for i in 0.. {
        let chosen_numbers = &board_draw[0..=i];

        for board in &boards {
            if is_board_solved(board, chosen_numbers) {
                return sum_board(board, chosen_numbers);
            }
        }
    }

    unreachable!();
}
