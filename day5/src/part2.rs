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

        if start_x == end_x || start_y == end_y {
            for x in start_x.min(end_x)..=end_x.max(start_x) {
                for y in start_y.min(end_y)..=end_y.max(start_y) {
                    *lines.entry((x, y)).or_default() += 1;
                }
            }
        } else {
            let mut current_x = start_x;
            let mut current_y = start_y;

            loop {
                *lines.entry((current_x, current_y)).or_default() += 1;

                // check to see if we have reached the destination
                if current_x == end_x {
                    break;
                }

                if current_x > end_x {
                    current_x -= 1;
                } else {
                    current_x += 1;
                }
                if current_y > end_y {
                    current_y -= 1;
                } else {
                    current_y += 1;
                }
            }
        }
    }

    lines.iter().filter(|(_, &d)| d > 1).count() as u32
}
