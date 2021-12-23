use crate::State;

pub fn parse_input(input: &str, part2: bool) -> State {
    let pre_rooms = input
        .lines()
        .skip(2)
        .map(|line| {
            line.chars()
                .filter(|c| *c != '#' && *c != ' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut rooms = vec![vec!['.'; 4]; 4];
    for (y, row) in pre_rooms.iter().enumerate().take(pre_rooms.len() - 1) {
        for (x, r) in rooms.iter_mut().enumerate().take(pre_rooms[y].len()) {
            r[pre_rooms[y].len() - 1 - y] = row[x];
        }
    }

    for r in rooms.iter_mut() {
        r.remove(0);
        r.remove(0);
    }

    if part2 {
        rooms[0].insert(1, 'D');
        rooms[0].insert(2, 'D');
        rooms[1].insert(1, 'B');
        rooms[1].insert(2, 'C');
        rooms[2].insert(1, 'A');
        rooms[2].insert(2, 'B');
        rooms[3].insert(1, 'C');
        rooms[3].insert(2, 'A');
    }

    let room_size = if part2 { 4 } else { 2 };
    State {
        rooms,
        buffer: ['.'; 7],
        room_size,
    }
}
