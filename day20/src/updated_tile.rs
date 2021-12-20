pub fn updated_tile(grid: &[Vec<u8>], enhancement: &[u8], r: usize, c: usize, val: u8) -> u8 {
    let ns = [
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ];
    let i = ns.iter().fold(0, |n, &(r, c)| {
        let x = *grid.get(r).and_then(|row| row.get(c)).unwrap_or(&val) as usize;
        n << 1 | x
    });

    enhancement[i]
}
