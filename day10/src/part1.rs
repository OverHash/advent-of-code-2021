pub fn get_first_invalid_character(string: &str) -> Option<char> {
    let mut stack = Vec::new();

    for char in string.chars() {
        match char {
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '<' => stack.push('>'),
            ']' | '}' | ')' | '>' if stack.pop() != Some(char) => return Some(char),
            _ => (),
        }
    }

    None
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let invalid_char = get_first_invalid_character(line);

            match invalid_char {
                None => 0,
                Some(char) => match char {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                },
            }
        })
        .sum()
}
