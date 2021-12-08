// huge credit to https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/08.rs
// since I failed to solve many parts to this problem
use itertools::Itertools;

fn calculate_digit(permutation: &[char], string: &str) -> Option<usize> {
    let decoded_string = string
        .chars()
        .map(|c| permutation[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();

    let digit = match decoded_string.as_str() {
        "abcdefg" => 8,
        "bcdef" => 5,
        "acdfg" => 2,
        "abcdf" => 3,
        "abd" => 7,
        "abcdef" => 9,
        "bcdefg" => 6,
        "abef" => 4,
        "abcdeg" => 0,
        "ab" => 1,
        _ => return None,
    };
    Some(digit)
}

fn try_permutation(
    permutation: &[char],
    signal_patterns: &[&str],
    digit_output: &[&str],
) -> Option<usize> {
    let is_invalid = signal_patterns
        .iter()
        .map(|signal| calculate_digit(permutation, signal))
        .any(|res| res.is_none());
    if is_invalid {
        return None;
    }

    let answer = digit_output
        .iter()
        .rev()
        .enumerate()
        .map(|(i, output)| calculate_digit(permutation, output).unwrap() * 10_usize.pow(i as u32))
        .sum();

    Some(answer)
}

pub fn solve(input: &str) -> u32 {
    let mut amt = 0;
    for line in input.lines() {
        let mut split = line.split(" | ");
        let signal_patterns = split.next().unwrap().split(' ').collect::<Vec<_>>();
        let digit_output = split.next().unwrap().split(' ').collect::<Vec<_>>();

        // try for all possible permutations
        let res = "abcdefg"
            .chars()
            .permutations(7)
            .find_map(|perm| try_permutation(&perm, &signal_patterns, &digit_output))
            .unwrap();

        amt += res;
    }

    amt as u32
}
