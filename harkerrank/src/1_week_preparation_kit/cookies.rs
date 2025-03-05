use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'cookies' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY A
 */

// K = 9;
// A = [7, 8, 9, 10, 11, 12];
fn cookies(k: i32, a: &[i32]) -> i32 {
  use std::cmp::Reverse;
  use std::collections::BinaryHeap;
  let mut heap: BinaryHeap<Reverse<i32>> = a.iter().map(|&x| Reverse(x)).collect();

  let mut iterations = 0;

  while heap.len() >= 2 {
    let Reverse(first) = heap.pop().unwrap();

    if first >= k {
      return iterations;
    }

    let Reverse(second) = heap.pop().unwrap();
    let new_cookie = first + 2 * second;
    heap.push(Reverse(new_cookie));
    iterations += 1;
  }

  if let Some(Reverse(last)) = heap.peek() {
    if *last >= k {
      return iterations;
    }
  }

  -1
}

fn main() {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

  let first_multiple_input: Vec<String> = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .split(' ')
    .map(|s| s.to_string())
    .collect();

  let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

  let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

  let A: Vec<i32> = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim_end()
    .split(' ')
    .map(|s| s.to_string().parse::<i32>().unwrap())
    .collect();

  let result = cookies(k, &A);

  writeln!(&mut fptr, "{}", result).ok();
}
