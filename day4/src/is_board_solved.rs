/// Checks to see if a given bingo board is solved either horizontally or vertically
pub fn is_board_solved(board: &[Vec<u32>], nums: &[u32]) -> bool {
    // check rows
    for row in board {
        let mut row_solved = true;
        for num in row {
            if !nums.contains(num) {
                row_solved = false;
                break;
            }
        }

        if row_solved {
            return true;
        }
    }

    // check columns
    for i in 0..5 {
        let mut column_solved = true;
        for row in board {
            if !nums.contains(&row[i]) {
                column_solved = false;
                break;
            }
        }

        if column_solved {
            return true;
        }
    }

    false
}
