use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str, iterations: u32) -> u64 {
    let (template, insertion_rules) = input.split_once("\n\n").unwrap();
    let insertion_rules = insertion_rules
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|l| (l.0, l.1.chars().next().unwrap()))
        .collect::<Vec<_>>();

    let mut result_char_counter: HashMap<char, u64> = HashMap::new();
    for char in template.chars() {
        *result_char_counter.entry(char).or_default() += 1;
    }
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();
    for (a, b) in template.chars().tuple_windows() {
        *pairs.entry((a, b)).or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_pairs: HashMap<(char, char), u64> = HashMap::new();
        for (pair, pair_amount) in &pairs {
            let str_pair = &format!("{}{}", pair.0, pair.1);
            for &(rule, output) in &insertion_rules {
                if str_pair != rule {
                    continue;
                }

                *result_char_counter.entry(output).or_default() += pair_amount;
                *new_pairs
                    .entry((rule.chars().next().unwrap(), output))
                    .or_default() += pair_amount;
                *new_pairs
                    .entry((output, rule.chars().nth(1).unwrap()))
                    .or_default() += pair_amount;

                break;
            }
        }
        pairs = new_pairs;
    }

    let most_common = result_char_counter
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1;
    let least_common = result_char_counter
        .iter()
        .max_by(|a, b| b.1.cmp(a.1))
        .unwrap()
        .1;

    most_common - least_common
}
