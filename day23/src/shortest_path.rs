use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::State;

pub fn shortest_path(s: State, f: State) -> i64 {
    let mut costs: HashMap<State, i64> = HashMap::default();

    let mut q = BinaryHeap::new();
    costs.insert(s.clone(), 0);
    q.push((0, s));
    while let Some((cost, grid)) = q.pop() {
        let cost = -cost;
        if cost != costs[&grid] {
            continue;
        }
        if grid == f {
            break;
        }
        for (transition, t_cost) in grid.transitions() {
            if let Some(&c) = costs.get(&transition) {
                if c <= t_cost + cost {
                    continue;
                }
            }
            costs.insert(transition.clone(), t_cost + cost);
            q.push((-(t_cost + cost), transition));
        }
    }
    *costs.get(&f).unwrap()
}
