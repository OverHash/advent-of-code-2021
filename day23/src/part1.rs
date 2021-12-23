use crate::{parse_input, shortest_path, State};

pub fn solve(input: &str) -> i64 {
    let start = parse_input(input, false);

    let expected = State {
        rooms: vec![
            vec!['A', 'A'],
            vec!['B', 'B'],
            vec!['C', 'C'],
            vec!['D', 'D'],
        ],
        buffer: ['.'; 7],
        room_size: 2,
    };

    shortest_path(start, expected)
}
