use crate::{parse_input, shortest_path, State};

pub fn solve(input: &str) -> i64 {
    let start = parse_input(input, true);

    let expected = State {
        rooms: vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'B', 'B'],
            vec!['C', 'C', 'C', 'C'],
            vec!['D', 'D', 'D', 'D'],
        ],
        buffer: ['.'; 7],
        room_size: 4,
    };

    shortest_path(start, expected)
}
