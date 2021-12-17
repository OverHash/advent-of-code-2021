use itertools::Itertools;

pub fn step(
    current_position: &(i32, i32),
    current_velocity: &(i32, i32),
) -> ((i32, i32), (i32, i32)) {
    let new_position = (
        current_position.0 + current_velocity.0,
        current_position.1 + current_velocity.1,
    );

    let new_velocity = (
        match current_velocity.0 {
            _ if current_velocity.0 < 0 => current_velocity.0 + 1,
            0 => 0,
            _ => current_velocity.0 - 1,
        },
        current_velocity.1 - 1,
    );

    (new_position, new_velocity)
}

fn is_solved(
    current_position: &(i32, i32),
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> bool {
    current_position.0 <= max_x
        && current_position.0 >= min_x
        && current_position.1 >= min_y
        && current_position.1 <= max_y
}

pub fn solve(input: &str) -> usize {
    let input = &input[13..];
    let (input_x, input_y) = input.split_once(", ").unwrap();
    let (input_x, input_y) = (&input_x[2..], &input_y[2..]);

    let (x_min, x_max) = input_x.split_once("..").unwrap();
    let (x_min, x_max) = (x_min.parse().unwrap(), x_max.parse().unwrap());

    let (y_min, y_max) = input_y.split_once("..").unwrap();
    let (y_min, y_max) = (y_min.parse::<i32>().unwrap(), y_max.parse().unwrap());

    (1..=x_max)
        .cartesian_product(-y_min.abs()..y_min.abs())
        .filter(|&(x, y)| {
            let mut current_position = (0, 0);
            let mut current_velocity = (x, y);

            while current_position.1 > y_min {
                let (new_position, new_velocity) = step(&current_position, &current_velocity);
                current_position = new_position;
                current_velocity = new_velocity;

                if is_solved(&current_position, x_min, x_max, y_min, y_max) {
                    return true;
                }
            }

            false
        })
        .count()
}
