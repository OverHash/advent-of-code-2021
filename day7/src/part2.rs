pub fn solve(input: &str) -> u32 {
    let crabs = input
        .split(',')
        .flat_map(|d| d.parse())
        .collect::<Vec<i32>>();

    crabs
        .iter()
        .map(|crab| {
            crabs
                .iter()
                .map(|other_crab| {
                    let crab_distance = (other_crab - crab).abs();
                    (crab_distance * (crab_distance + 1) / 2) as u32
                })
                .sum()
        })
        .min()
        .unwrap()
}
