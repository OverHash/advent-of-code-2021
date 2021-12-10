pub fn get_missing_characters(string: &str) -> Option<Vec<char>> {
    let mut stack = Vec::new();

    for char in string.chars() {
        match char {
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '<' => stack.push('>'),
            ']' | '}' | ')' | '>' if stack.pop() != Some(char) => return None,
            _ => (),
        }
    }

    Some(stack.into_iter().rev().collect())
}

pub fn solve(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .flat_map(|line| {
            let invalid_char = get_missing_characters(line);

            invalid_char.map(|missing_chars| {
                missing_chars
                    .iter()
                    .map(|char| match char {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    })
                    .fold(0, |acc, n| acc * 5 + n)
            })
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2]
}
