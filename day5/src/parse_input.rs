use regex::Regex;

pub struct ParsedInput {
    pub start_x: u32,
    pub start_y: u32,

    pub finish_x: u32,
    pub finish_y: u32,
}

pub fn parse_input(input: &str) -> Vec<ParsedInput> {
    let input_regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").expect("Invalid input regex");

    input
        .lines()
        .map(|line| {
            let input = input_regex.captures(line).unwrap();
            let mut numbers = input.iter().skip(1);

            ParsedInput {
                start_x: numbers.next().unwrap().unwrap().as_str().parse().unwrap(),
                start_y: numbers.next().unwrap().unwrap().as_str().parse().unwrap(),

                finish_x: numbers.next().unwrap().unwrap().as_str().parse().unwrap(),
                finish_y: numbers.next().unwrap().unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}
