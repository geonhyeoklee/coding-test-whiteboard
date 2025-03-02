use std::io::{self, BufRead};

/*
 * Complete the 'minimumBribes' function below.
 *
 * The function accepts INTEGER_ARRAY q as parameter.
 */

fn minimumBribes(q: &[i32]) {}

fn main() {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  let t = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim()
    .parse::<i32>()
    .unwrap();

  for _ in 0..t {
    let n = stdin_iterator
      .next()
      .unwrap()
      .unwrap()
      .trim()
      .parse::<i32>()
      .unwrap();

    let q: Vec<i32> = stdin_iterator
      .next()
      .unwrap()
      .unwrap()
      .trim_end()
      .split(' ')
      .map(|s| s.to_string().parse::<i32>().unwrap())
      .collect();

    minimumBribes(&q);
  }
}
