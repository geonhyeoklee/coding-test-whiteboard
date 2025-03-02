use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'superDigit' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING n
 *  2. INTEGER k
 */

// The recursive solution
// fn superDigit(n: &str, k: i32) -> i32 {
//   let mut sum: u64 = 0;

//   for c in n.chars() {
//     sum += c.to_digit(10).unwrap() as u64;
//   }

//   sum *= k as u64;

//   if sum < 10 {
//     return sum as i32;
//   }

//   superDigit(&sum.to_string(), 1)
// }

// The stack solution
fn superDigit(n: &str, k: i32) -> i32 {
  let mut stack = vec![(n.to_string(), k)];

  let mut result = 0;

  while let Some((n, k)) = stack.pop() {
    let mut sum: u64 = 0;

    for c in n.chars() {
      sum += c.to_digit(10).unwrap() as u64;
    }

    sum *= k as u64;

    if sum < 10 {
      result = sum;
      break;
    }

    stack.push((sum.to_string(), 1));
  }

  result as i32
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

  let n = &first_multiple_input[0];

  let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

  let result = superDigit(n, k);

  writeln!(&mut fptr, "{}", result).ok();
}
