use std::collections::HashSet;

use itertools::Itertools;

use crate::get_rotation::get_rotation;

pub fn merge_scan(total_scan: &mut HashSet<[i32; 3]>, b: &[[i32; 3]], rot: u8) -> Option<[i32; 3]> {
    let b = b.iter().map(|&v| get_rotation(v, rot)).collect::<Vec<_>>();

    let distances = total_scan
        .iter()
        .cartesian_product(&b)
        .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);

    for [dx, dy, dz] in distances {
        let translated = b.iter().map(|[x3, y3, z3]| [x3 + dx, y3 + dy, z3 + dz]);
        if translated
            .clone()
            .filter(|v| total_scan.contains(v))
            .count()
            >= 12
        {
            total_scan.extend(translated);
            return Some([dx, dy, dz]);
        }
    }
    None
}
