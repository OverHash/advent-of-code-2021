use std::collections::{HashMap, HashSet};

fn search<'a>(
    cave: &'a str,
    neighbors: &HashMap<&'a str, Vec<&'a str>>,
    seen: &HashSet<&'a str>,
) -> u32 {
    if cave == "end" {
        return 1;
    }
    if seen.contains(cave) {
        if cave == "start" {
            return 0;
        }

        if cave.to_ascii_lowercase() == cave {
            return 0;
        }
    }

    neighbors
        .get(cave)
        .unwrap()
        .iter()
        .map(|neighbor| {
            let mut cloned_seen = seen.clone();
            cloned_seen.insert(cave);
            search(neighbor, neighbors, &cloned_seen)
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

    search("start", &neighbors, &HashSet::new())
}
