use crate::{find_scan_merge::find_scan_merge, parse_input::parse_input};

pub fn solve(input: &str) -> u32 {
    let mut scans = parse_input(input);

    let mut total_scan = scans.swap_remove(0).into_iter().collect();
    let mut dists = Vec::new();
    while let Some((d, i)) = find_scan_merge(&scans, &mut total_scan) {
        dists.push(d);
        scans.swap_remove(i);
    }

    total_scan.len() as u32
}
