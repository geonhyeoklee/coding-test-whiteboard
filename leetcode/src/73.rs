struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut target_rows = HashSet::new();
        let mut target_cols = HashSet::new();

        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                if *val == 0 {
                    target_rows.insert(row_idx);
                    target_cols.insert(col_idx);
                }
            }
        }

        for (row_idx, row) in matrix.iter_mut().enumerate() {
            for (col_idx, element) in row.iter_mut().enumerate() {
                if target_rows.get(&row_idx).is_some() || target_cols.get(&col_idx).is_some() {
                    *element = 0;
                }
            }
        }
    }
}
fn main() {
    let mut matrix = vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1], vec![1, 1, 1, 1]];

    Solution::set_zeroes(&mut matrix);
}
