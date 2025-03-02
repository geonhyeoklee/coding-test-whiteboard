enum Direction {
    Up,
    Down,
    Right,
    Left,
}
struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut direction = Direction::Right;
        let mut round = 0;
        let mut row = 0;
        let mut col = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut flatten_matrix = vec![];

        for _ in 0..m * n {
            flatten_matrix.push(matrix[row][col]);

            match direction {
                Direction::Up => {
                    if row == round + 1 {
                        col += 1;
                        direction = Direction::Right;
                        round += 1;
                    } else {
                        row -= 1;
                    }
                }
                Direction::Down => {
                    if row + 1 == m - round {
                        col -= 1;
                        direction = Direction::Left;
                    } else {
                        row += 1;
                    }
                }
                Direction::Right => {
                    if col + 1 == n - round {
                        row += 1;
                        direction = Direction::Down;
                    } else {
                        col += 1;
                    }
                }
                Direction::Left => {
                    if col == round {
                        row -= 1;
                        direction = Direction::Up;
                    } else {
                        col -= 1;
                    }
                }
            }
        }

        flatten_matrix
    }
}
fn main() {
    let board = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let result = Solution::spiral_order(board);

    println!("{:?}", result);
}
