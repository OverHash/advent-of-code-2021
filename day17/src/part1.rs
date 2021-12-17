pub fn solve(input: &str) -> i32 {
    // remove "target area: "
    let input = &input[13..];
    // get "y=-10..5"
    let (_, input_y) = input.split_once(", ").unwrap();
    // get "-10..5"
    let input_y = &input_y[2..];

    let y_min = input_y.split("..").next().unwrap();
    let y_min: i32 = y_min.parse().unwrap();

    (y_min) * (y_min + 1) / 2
}
