const WEIGHT: [i64; 4] = [1, 10, 100, 1000];
const BUF_WEIGHT: [i64; 7] = [0, 1, 3, 5, 7, 9, 10];

fn weight(c: char) -> i64 {
    WEIGHT[(c as u8 - b'A') as usize]
}

fn buf_traversal_cost(i: usize, j: usize, c: char) -> i64 {
    (BUF_WEIGHT[i] - BUF_WEIGHT[j]).abs() * WEIGHT[(c as u8 - b'A') as usize]
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub rooms: Vec<Vec<char>>,
    pub buffer: [char; 7],
    pub room_size: usize,
}

impl State {
    fn is_valid_room(&self, i: usize) -> bool {
        self.rooms[i]
            .iter()
            .all(|&c| i == (c as u8 - b'A') as usize)
    }

    fn entry_cost(&self, i: usize) -> i64 {
        (self.room_size - self.rooms[i].len()) as i64 * WEIGHT[i]
    }

    fn exit_cost(&self, i: usize, c: char) -> i64 {
        (self.room_size - self.rooms[i].len()) as i64 * weight(c)
    }

    fn transition_room_to_buffer(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..4 {
            if self.is_valid_room(i) {
                continue;
            }
            let mut next = self.clone();
            let c = next.rooms[i].pop().unwrap();
            for j in (0..=i + 1).rev() {
                let cost = buf_traversal_cost(j, i + 1, c) + weight(c) + next.exit_cost(i, c);
                if next.buffer[j] == '.' {
                    next.buffer[j] = c;
                    res.push((next.clone(), cost));
                    next.buffer[j] = '.';
                } else {
                    break;
                }
            }
            for j in i + 2..7 {
                let cost = buf_traversal_cost(i + 2, j, c) + weight(c) + next.exit_cost(i, c);
                if next.buffer[j] == '.' {
                    next.buffer[j] = c;
                    res.push((next.clone(), cost));
                    next.buffer[j] = '.';
                } else {
                    break;
                }
            }
        }
        res
    }

    fn transition_buffer_to_room(&self) -> Vec<(State, i64)> {
        let mut res = vec![];
        for i in 0..7 {
            if self.buffer[i] == '.' {
                continue;
            }
            let r = (self.buffer[i] as u8 - b'A') as usize;
            if !self.is_valid_room(r) {
                continue;
            }
            if i <= r + 1 {
                if (i + 1..=r + 1).all(|i| self.buffer[i] == '.') {
                    let mut next = self.clone();
                    let c = buf_traversal_cost(i, r + 1, next.buffer[i])
                        + weight(next.buffer[i])
                        + self.entry_cost(r);
                    next.rooms[r].push(next.buffer[i]);
                    next.buffer[i] = '.';
                    res.push((next, c));
                }
            } else if (r + 2..i).all(|i| self.buffer[i] == '.') {
                let mut next = self.clone();
                let c = buf_traversal_cost(r + 2, i, next.buffer[i])
                    + weight(next.buffer[i])
                    + self.entry_cost(r);
                next.rooms[r].push(next.buffer[i]);
                next.buffer[i] = '.';
                res.push((next, c));
            }
        }
        res
    }

    pub fn transitions(&self) -> Vec<(State, i64)> {
        let mut res = self.transition_room_to_buffer();
        res.append(&mut self.transition_buffer_to_room());
        res
    }
}
