use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingSort' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn countingSort(arr: &[i32]) -> Vec<i32> {
  let mut arr = arr.to_vec();
  arr.sort();

  let mut counts = vec![0; 100];

  for i in 0..arr.len() {
    counts[arr[i] as usize] += 1;
  }

  counts
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

  let arr: Vec<i32> = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim_end()
    .split(' ')
    .map(|s| s.to_string().parse::<i32>().unwrap())
    .collect();

  let result = countingSort(&arr);

  for i in 0..result.len() {
    write!(&mut fptr, "{}", result[i]).ok();

    if i != result.len() - 1 {
      write!(&mut fptr, " ").ok();
    }
  }

  writeln!(&mut fptr).ok();
}
