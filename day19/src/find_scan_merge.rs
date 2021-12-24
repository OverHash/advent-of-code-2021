use std::collections::HashSet;

use crate::merge_scan::merge_scan;

pub fn find_scan_merge(
    scans: &[Vec<[i32; 3]>],
    total_scan: &mut HashSet<[i32; 3]>,
) -> Option<([i32; 3], usize)> {
    (0..scans.len()).find_map(|i| {
        (0..24)
            .find_map(|rot| merge_scan(total_scan, &scans[i], rot))
            .map(|d| (d, i))
    })
}
