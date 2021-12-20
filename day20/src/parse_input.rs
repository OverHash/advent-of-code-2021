pub fn parse_input(input: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let (enhancement, input) = input.split_once("\n\n").unwrap();

    (
        enhancement.chars().map(|c| (c == '#') as u8).collect(),
        input
            .lines()
            .map(|l| l.chars().map(|c| (c == '#') as u8).collect())
            .collect(),
    )
}
