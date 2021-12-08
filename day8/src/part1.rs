pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // skip the first one as that is the wires
            // which we do not need to solve for in this part of the problem
            // and then split by the space to get the output display wires
            let digit_output = line.split(" | ").nth(1).unwrap().split(' ');

            digit_output
                .filter(|&output| [2, 4, 3, 7].contains(&output.len()))
                .count() as u32
        })
        .sum()
}
