fn get_reading(numbers: &[usize], bit_position: usize, most_common: bool) -> usize {
    let len = numbers.len();

    let one_bits = numbers
        .iter()
        .filter(|&n| n & (1 << bit_position) > 0)
        .count();
    let zero_bits = len - one_bits;

    let keep_bit = if most_common {
        if one_bits >= zero_bits {
            1
        } else {
            0
        }
    } else if one_bits >= zero_bits {
        0
    } else {
        1
    };

    let filtered = numbers
        .iter()
        .filter(|&n| {
            if keep_bit == 1 {
                (n & (1 << bit_position)) > 0
            } else {
                (n & (1 << bit_position)) == 0
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    if filtered.len() == 1 {
        return filtered[0];
    }

    get_reading(&filtered, bit_position - 1, most_common)
}

pub fn solve(input: &str) -> u32 {
    let input = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

    let oxygen_rating = get_reading(&input, 11, true);
    let co2_rating = get_reading(&input, 11, false);

    (oxygen_rating * co2_rating) as u32
}
