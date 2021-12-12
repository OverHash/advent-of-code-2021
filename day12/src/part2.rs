use std::collections::{HashMap, HashSet};

fn search<'a>(
    cave: &'a str,
    can_revisit: bool,
    neighbors: &HashMap<&'a str, Vec<&'a str>>,
    seen: &HashSet<&'a str>,
) -> u32 {
    if cave == "end" {
        return 1;
    }

    let mut can_revisit = can_revisit;

    if seen.contains(cave) {
        if cave == "start" {
            return 0;
        }

        if cave.to_ascii_lowercase() == cave {
            if !can_revisit {
                return 0;
            }

            can_revisit = false;
        }
    }

    neighbors
        .get(cave)
        .unwrap()
        .iter()
        .map(|neighbor| {
            let mut cloned_seen = seen.clone();
            cloned_seen.insert(cave);
            search(neighbor, can_revisit, neighbors, &cloned_seen)
        })
        .sum()
}

pub fn solve(input: &str) -> u32 {
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut split = line.split('-');
        let a = split.next().unwrap();
        let b = split.next().unwrap();

        neighbors.entry(a).or_default().push(b);
        neighbors.entry(b).or_default().push(a);
    }

    search("start", true, &neighbors, &HashSet::new())
}
