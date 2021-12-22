type Block = Vec<(i128, i128)>;

struct Event {
    op: String,
    block: Block,
    time: i128,
    idx: usize,
}

fn get_volume(op_coords: &[(String, Block, usize)]) -> i128 {
    if op_coords.is_empty() {
        return 0;
    }
    if op_coords[0].1.is_empty() {
        let max_idx = op_coords.iter().max_by_key(|(_, _, idx)| *idx).unwrap();
        if max_idx.0 == "on" {
            return 1;
        } else {
            return 0;
        }
    }

    let mut events = vec![];
    for (op, coords, idx) in op_coords {
        events.push(Event {
            op: op.to_string(),
            block: coords[..coords.len() - 1].to_vec(),
            time: coords.last().unwrap().0,
            idx: *idx,
        });
        events.push(Event {
            op: op.to_string(),
            block: coords[..coords.len() - 1].to_vec(),
            time: coords.last().unwrap().1,
            idx: *idx,
        });
    }
    events.sort_by_key(|e| e.time);

    let mut squares: Vec<(String, Block, usize)> = vec![];
    let mut time = 0;
    let mut area = 0;
    let mut volume = 0;

    for event in events {
        let cur_time = event.time;
        volume += (cur_time - time) * area;

        if !squares.iter().any(|(_, _, idx)| *idx == event.idx) {
            squares.push((event.op.clone(), event.block.clone(), event.idx));
        } else {
            squares.retain(|(_, _, idx)| *idx != event.idx);
        }

        area = get_volume(&squares);
        time = cur_time;
    }

    volume
}

pub fn solve(input: &str) -> i128 {
    let input = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (op, rest) = line.split_once(" ").unwrap();
            let coords = rest
                .split(',')
                .map(|s| {
                    let (_, s) = s.split_once("=").unwrap();
                    let (l, r) = s.split_once("..").unwrap();
                    (l.parse::<i128>().unwrap(), r.parse::<i128>().unwrap() + 1)
                })
                .collect::<Vec<_>>();
            (op.to_string(), coords, idx)
        })
        .collect::<Vec<_>>();

    get_volume(&input)
}
