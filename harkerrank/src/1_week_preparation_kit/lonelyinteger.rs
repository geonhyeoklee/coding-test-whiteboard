use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'lonelyinteger' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn lonelyinteger(a: &[i32]) -> i32 {
  let a = a.to_vec();
  let mut cache = std::collections::HashMap::new();

  for i in a.iter() {
    cache.entry(i).and_modify(|v| *v += 1).or_insert(1);
  }

  let mut result = 0;

  for (k, v) in cache.iter() {
    if *v == 1 {
      result = **k;
    }
  }

  result
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

  let a: Vec<i32> = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim_end()
    .split(' ')
    .map(|s| s.to_string().parse::<i32>().unwrap())
    .collect();

  let result = lonelyinteger(&a);

  writeln!(&mut fptr, "{}", result).ok();
}
