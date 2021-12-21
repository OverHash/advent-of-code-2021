pub fn solve(input: &str) -> usize {
    let (player1, player2) = input.split_once("\n").unwrap();
    let player1 = player1.chars().nth(28).unwrap().to_digit(10).unwrap() as usize;
    let player2 = player2.chars().nth(28).unwrap().to_digit(10).unwrap() as usize;

    let mut positions = vec![player1, player2];
    let mut scores = vec![0; 2];

    let mut die_rolls = 0;
    let mut current_die_val = (1..=100).cycle();

    // which player turn it is
    let mut current_player = 0;
    loop {
        let steps = (0..3)
            .map(|_| current_die_val.next().unwrap())
            .sum::<usize>();
        die_rolls += 3;

        positions[current_player] = ((positions[current_player] + steps - 1) % 10) + 1;
        scores[current_player] += positions[current_player];

        if scores[current_player] > 1000 {
            break;
        }

        current_player = (current_player + 1) % 2;
    }

    scores[(current_player + 1) % 2] * die_rolls
}
