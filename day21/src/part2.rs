use std::collections::HashMap;

type Cache = HashMap<(usize, usize, usize, usize), (usize, usize)>;

fn quantum_game(
    cache: &mut Cache,
    p1_score: usize,
    p2_score: usize,
    p1_pos: usize,
    p2_pos: usize,
) -> (usize, usize) {
    if p2_score >= 21 {
        return (0, 1);
    }
    if let Some(&score) = cache.get(&(p1_score, p2_score, p1_pos, p2_pos)) {
        return score;
    }

    let mut score = (0, 0);
    for (die, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let pos1 = p1_pos + die - if p1_pos + die > 10 { 10 } else { 0 };
        let (s1, s2) = quantum_game(cache, p2_score, p1_score + pos1, p2_pos, pos1);
        score = (score.0 + s2 * times, score.1 + s1 * times);
    }

    cache.insert((p1_score, p2_score, p1_pos, p2_pos), score);
    score
}

pub fn solve(input: &str) -> usize {
    let (player1, player2) = input.split_once("\n").unwrap();
    let player1 = player1.chars().nth(28).unwrap().to_digit(10).unwrap() as usize;
    let player2 = player2.chars().nth(28).unwrap().to_digit(10).unwrap() as usize;

    let (p1_wins, p2_wins) = quantum_game(&mut HashMap::new(), 0, 0, player1, player2);
    p1_wins.max(p2_wins)
}
