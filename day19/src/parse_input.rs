pub fn parse_input(input: &str) -> Vec<Vec<[i32; 3]>> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|l| {
                    let mut coordinates = l.split(',').map(|d| d.parse().unwrap());
                    let x = coordinates.next().unwrap();
                    let y = coordinates.next().unwrap();
                    let z = coordinates.next().unwrap();

                    [x, y, z]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
