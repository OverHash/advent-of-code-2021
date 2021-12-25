use crate::rotation::Rotation;

pub fn parse_input(input: &str) -> Vec<Vec<Rotation>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Rotation::None,
                    '>' => Rotation::Right,
                    'v' => Rotation::Down,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}
