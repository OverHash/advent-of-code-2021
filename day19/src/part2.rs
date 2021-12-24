use itertools::Itertools;

use crate::{find_scan_merge::find_scan_merge, parse_input::parse_input};

pub fn solve(input: &str) -> u32 {
    let mut scans = parse_input(input);

    let mut total_scan = scans.swap_remove(0).into_iter().collect();
    let mut dists = Vec::new();

    while let Some((d, i)) = find_scan_merge(&scans, &mut total_scan) {
        dists.push(d);
        scans.swap_remove(i);
    }

    dists
        .iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap() as u32
}
