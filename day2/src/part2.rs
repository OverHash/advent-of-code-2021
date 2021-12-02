pub fn solve(input: &str) -> i64 {
    let mut x = 0;
    let mut aim = 0;
    let mut depth = 0;

    for line in input.lines().map(|line| line.split(' ')) {
        let line: Vec<_> = line.collect();
        let instruction = line[0];
        let amt = line[1].parse::<i64>().unwrap();
        match instruction {
            "forward" => {
                x += amt;
                depth += aim * amt;
            }
            "down" => {
                aim += amt;
            }
            "up" => {
                aim -= amt;
            }
            _ => (),
        }
    }

    x * depth
}
