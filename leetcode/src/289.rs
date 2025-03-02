struct Solution;

impl Solution {
  pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let mut neighbor_count = 0;
    let mut new_board: Vec<Vec<i32>> = vec![];

    for (row_idx, row) in board.clone().into_iter().enumerate() {
      let mut new_row: Vec<i32> = vec![];

      for (cell_idx, cell) in row.clone().into_iter().enumerate() {
        for neighbor_row_idx in -1..2 as i32 {
          let row_target_idx = row_idx as i32 + neighbor_row_idx;

          if board.get(row_target_idx as usize).is_some() {
            let row = board.get(row_target_idx as usize).unwrap();

            for neighbor_cell_idx in -1..2 as i32 {
              let target = cell_idx as i32 + neighbor_cell_idx;

              neighbor_count += if row.get(target as usize).is_some() {
                row.get(target as usize).unwrap()
              } else {
                &0
              }
            }
          }
        }

        neighbor_count -= cell;

        match neighbor_count {
          2 => {
            new_row.push(cell);
          }
          3 => {
            new_row.push(1);
          }
          _ => new_row.push(0),
        }

        neighbor_count = 0;
      }

      new_board.push(new_row.clone());
    }

    *board = new_board;
  }
}

fn main() {
  let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
  Solution::game_of_life(&mut board);
}
