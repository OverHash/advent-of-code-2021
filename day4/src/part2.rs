use crate::is_board_solved::is_board_solved;
use crate::sum_board::sum_board;

pub fn solve(input: &str) -> u32 {
    let mut input = input.split("\n\n");
    let board_draw = input
        .next()
        .unwrap()
        .split(',')
        .flat_map(|d| d.parse())
        .collect::<Vec<_>>();

    let mut boards = input
        .map(|board| {
            board
                .lines()
                .map(|row| row.split_whitespace().flat_map(|d| d.parse()).collect())
                .collect::<Vec<Vec<_>>>()
        })
        .collect::<Vec<_>>();

    for i in 5.. {
        let chosen_numbers = &board_draw[0..=i];

        // check if we are the last board and are solved to calculate the sum
        if is_board_solved(&boards[0], chosen_numbers) && boards.len() == 1 {
            return sum_board(&boards[0], chosen_numbers);
        }

        boards.retain(|board| !is_board_solved(board, chosen_numbers));
    }

    unreachable!();
}
