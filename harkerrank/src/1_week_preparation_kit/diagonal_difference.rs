use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

// [ [1, 2, 3], [4, 5, 6], [9, 8, 9] ]
// 1 + 5 + 9 = 15
// 3 + 5 + 9 = 17
// 15 - 17 = -2

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
  let indexed = arr
    .iter()
    .enumerate()
    .map(|(idx, _)| idx)
    .collect::<Vec<usize>>();
  let mut left_to_right = 0;
  let mut right_to_left = 0;

  for (idx, vec) in arr.iter().enumerate() {
    left_to_right += vec[indexed[idx]];
  }

  for (idx, vec) in arr.iter().enumerate() {
    right_to_left += vec[indexed[arr.len() - 1 - idx]];
  }

  i32::abs(left_to_right - right_to_left)
}

fn main() {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

  let n = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim()
    .parse::<i32>()
    .unwrap();

  let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

  for i in 0..n as usize {
    arr.push(Vec::with_capacity(n as usize));

    arr[i] = stdin_iterator
      .next()
      .unwrap()
      .unwrap()
      .trim_end()
      .split(' ')
      .map(|s| s.to_string().parse::<i32>().unwrap())
      .collect();
  }

  let result = diagonalDifference(&arr);

  writeln!(&mut fptr, "{}", result).ok();
}
