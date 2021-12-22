use std::collections::HashMap;

fn do_instruction(input: &str, grid: &mut HashMap<(i32, i32, i32), bool>) {
    let (instruction_type, instructions) = input.split_once(' ').unwrap();

    let mut instructions = instructions.split(',');
    let x = &instructions.next().unwrap()[2..];
    let y = &instructions.next().unwrap()[2..];
    let z = &instructions.next().unwrap()[2..];

    let (x1, x2) = x.split_once("..").unwrap();
    let (y1, y2) = y.split_once("..").unwrap();
    let (z1, z2) = z.split_once("..").unwrap();

    let (x1, x2) = (x1.parse::<i32>().unwrap(), x2.parse().unwrap());
    let (y1, y2) = (y1.parse::<i32>().unwrap(), y2.parse().unwrap());
    let (z1, z2) = (z1.parse::<i32>().unwrap(), z2.parse().unwrap());

    for x in (x1.min(x2).max(-50))..=(x1.max(x2).min(50)) {
        for y in (y1.min(y2).max(-50))..=(y1.max(y2).min(50)) {
            for z in (z1.min(z2).max(-50))..=(z1.max(z2).min(50)) {
                *grid.entry((x, y, z)).or_default() = match instruction_type {
                    "on" => true,
                    "off" => false,
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn solve(input: &str) -> usize {
    let input = input.lines();

    let mut grid = HashMap::new();

    input.for_each(|l| do_instruction(l, &mut grid));

    grid.iter().filter(|&(_, &v)| v).count()
}
