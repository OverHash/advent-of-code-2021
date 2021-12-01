pub fn solve(input: &str) -> u32 {
    let input = input.lines().flat_map(|line| line.parse());

    input
        .fold((0, u32::MAX), |(amt, prev), curr| {
            if curr > prev {
                return (amt + 1, curr);
            }

            (amt, curr)
        })
        .0
}
