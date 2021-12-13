pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub enum FoldLocation {
    X(u32),
    Y(u32),
}

pub struct ParsedInput {
    pub coordinates: Vec<Point>,
    pub fold_coordinates: Vec<FoldLocation>,
}

pub fn parse_input(input: &str) -> ParsedInput {
    let mut separator = input.split("\n\n");
    let coordinates = separator.next().unwrap();
    let fold_coordinates = separator.next().unwrap();

    let coordinates = coordinates
        .lines()
        .map(|l| {
            let mut separator = l.split(',');
            let x = separator.next().unwrap().parse().unwrap();
            let y = separator.next().unwrap().parse().unwrap();

            Point { x, y }
        })
        .collect();

    let fold_coordinates = fold_coordinates
        .lines()
        .map(|l| {
            let l = l.split(' ').nth(2).unwrap();
            let mut separator = l.split('=');
            let coordinate = separator.next().unwrap();
            let value = separator.next().unwrap().parse().unwrap();

            match coordinate {
                "x" => FoldLocation::X(value),
                "y" => FoldLocation::Y(value),
                _ => unreachable!(),
            }
        })
        .collect();

    ParsedInput {
        coordinates,
        fold_coordinates,
    }
}
