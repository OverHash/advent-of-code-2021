/// Sums the unmarked numbers for a given board
/// Multiplies the result by the last number in the chosen number list
pub fn sum_board(board: &[Vec<u32>], chosen_numbers: &[u32]) -> u32 {
    let mut sum = 0;
    for row in board {
        for num in row {
            if !chosen_numbers.contains(num) {
                sum += num;
            }
        }
    }

    return sum * chosen_numbers.last().unwrap();
}
