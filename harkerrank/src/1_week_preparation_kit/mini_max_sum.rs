use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

// [1,3,5,7,9]
// 16
// 24

fn miniMaxSum(arr: &[i32]) {
  let mut min: i64 = 0;
  let mut max: i64 = 0;
  let mut arr = arr.to_vec();
  arr.sort();

  for i in 0..4 {
    min += arr[i] as i64;
    max += arr[arr.len() - 1 - i] as i64;
  }

  println!("{} {}", min, max);
}

fn main() {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  let arr: Vec<i32> = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim_end()
    .split(' ')
    .map(|s| s.to_string().parse::<i32>().unwrap())
    .collect();

  miniMaxSum(&arr);
}
