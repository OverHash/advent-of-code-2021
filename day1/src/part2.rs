fn sum(slice: &[u32]) -> u32 {
    let mut total = 0;

    for i in slice {
        total += i;
    }

    total
}

pub fn solve(input: &str) -> u32 {
    let input = input
        .lines()
        .flat_map(|line| line.parse::<u32>())
        .collect::<Vec<_>>();
    let mut input = input.windows(3);

    let mut previous = sum(input.next().unwrap());
    let mut increased = 0;
    for next in input {
        let next = sum(next);
        if next > previous {
            increased += 1;
        }

        previous = next;
    }

    increased
}
