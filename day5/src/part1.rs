use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let input = input.lines();

    let mut lines: HashMap<(u32, u32), u32> = HashMap::new();
    for line in input {
        let mut sides = line.split(" -> ");
        let mut start_nums = sides
            .next()
            .unwrap()
            .split(',')
            .flat_map(|n| n.parse::<u32>());
        let mut end_nums = sides
            .next()
            .unwrap()
            .split(',')
            .flat_map(|n| n.parse::<u32>());

        let start_x = start_nums.next().unwrap();
        let start_y = start_nums.next().unwrap();

        let end_x = end_nums.next().unwrap();
        let end_y = end_nums.next().unwrap();

        if !(start_x == end_x || start_y == end_y) {
            continue;
        }

        for x in start_x.min(end_x)..=start_x.max(end_x) {
            for y in start_y.min(end_y)..=start_y.max(end_y) {
                *lines.entry((x, y)).or_default() += 1;
            }
        }
    }

    lines.iter().filter(|(_, &d)| d > 1).count() as u32
}
