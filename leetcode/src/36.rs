use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_set: HashSet<(usize, u32)> = HashSet::new();
        let mut col_set: HashSet<(usize, u32)> = HashSet::new();
        let mut sub_box_sets = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                if let Some(digit) = board[i][j].to_digit(10) {
                    if !row_set.insert((i, digit)) {
                        return false;
                    }

                    if !col_set.insert((j, digit)) {
                        return false;
                    }

                    let sub_box_idx = 3 * (i / 3) + (j / 3);
                    if !sub_box_sets[sub_box_idx].insert(digit) {
                        return false;
                    }
                }
                col_set.clear()
            }
            row_set.clear()
        }

        true
    }
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let result = Solution::is_valid_sudoku(board);

    println!("{:?}", result);
}
