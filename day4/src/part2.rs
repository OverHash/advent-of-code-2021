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

    let mut boards = input
        .map(|board| {
            board
                .lines()
                .map(|row| row.split_whitespace().flat_map(|d| d.parse()).collect())
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    loop {
        let chosen_numbers = &board_draw[0..=i];

        let cloned_boards = boards.to_owned();

        let winners = cloned_boards
            .iter()
            .filter(|board| is_board_solved(board, chosen_numbers))
            .collect::<Vec<_>>();

        for winner in winners {
            if boards.len() == 1 {
                return sum_board(winner, chosen_numbers);
            }

            let position = boards.iter().position(|board| board == winner).unwrap();
            boards.swap_remove(position);
        }

        i += 1;
    }
}
