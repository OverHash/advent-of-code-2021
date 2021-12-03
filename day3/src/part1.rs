pub fn solve(input: &str) -> u32 {
    let input: Vec<usize> = input
        .lines()
        .map(|num| usize::from_str_radix(num, 2).unwrap())
        .collect();
    let len = input.len();

    // gamma rate = most common bit
    // epsilon rate = least common bit
    let mut gamma_rate: u32 = 0;

    for i in (0..12).rev() {
        let count = input.iter().filter(|&n| (n & (1 << i)) > 0).count();

        if count > len / 2 {
            gamma_rate += 1 << i;
        }
    }

    // 0xFFF = 111_111_111_111 in binary
    gamma_rate * (!gamma_rate & 0xFFF)
}
