pub fn solve(initial_fishes: Vec<u8>, days: u32) -> u64 {
    let mut days_remaining = [0; 9];

    for fish in initial_fishes {
        days_remaining[fish as usize] += 1;
    }

    for _ in 0..days {
        let new_fish = days_remaining[0];

        for i in 1..9 {
            days_remaining[i - 1] = days_remaining[i];
        }

        days_remaining[6] += new_fish;
        days_remaining[8] = new_fish;
    }

    days_remaining.iter().sum()
}
