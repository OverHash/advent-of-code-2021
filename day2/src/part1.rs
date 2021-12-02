pub fn solve(input: &str) -> u32 {
    let mut x = 0;
    let mut y = 0;

    for line in input.lines().map(|line| line.split(' ')) {
        let line: Vec<_> = line.collect();

        let instruction = line[0];
        let amt: u32 = line[1].parse().unwrap();

        match instruction {
            "forward" => {
                x += amt;
            }
            "down" => {
                y += amt;
            }
            "up" => {
                y -= amt;
            }
            _ => (),
        }
    }

    x * y
}
