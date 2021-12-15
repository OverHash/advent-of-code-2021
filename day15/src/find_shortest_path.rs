use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other_node: &Self) -> Ordering {
        other_node
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other_node.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_shortest_path(grid: &[Vec<u32>]) -> u32 {
    let end_position = (grid.len() - 1, grid[0].len() - 1);
    let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        position: (0, 0),
        cost: 0,
    });

    while let Some(Node {
        cost,
        position: (x, y),
    }) = queue.pop()
    {
        // check if we have reached the end
        if (x, y) == end_position {
            return cost;
        }

        // check if the cost is greater than max
        if cost > dist[x][y] {
            continue;
        }

        // iterate over all neighbors
        for (x1, y1) in [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ] {
            // check is a valid coordinate
            if grid.get(x1).and_then(|row| row.get(y1)).is_none() {
                continue;
            }

            let next_node = Node {
                cost: cost + grid[x1][y1],
                position: (x1, y1),
            };

            // check if this is cheaper
            if next_node.cost < dist[x1][y1] {
                dist[x1][y1] = next_node.cost;
                queue.push(next_node);
            }
        }
    }

    unreachable!()
}
