/// Checks to see if a given bingo board is solved either horizontally or vertically
pub fn is_board_solved(board: &[Vec<u32>], nums: &[u32]) -> bool {
    // check rows
    for row in board {
        if row.iter().all(|num| nums.contains(num)) {
            return true;
        }
    }

    // check columns
    for i in 0..5 {
        if board.iter().all(|row| nums.contains(&row[i])) {
            return true;
        }
    }

    false
}
