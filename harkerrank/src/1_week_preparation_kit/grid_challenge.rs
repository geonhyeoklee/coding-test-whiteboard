use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'gridChallenge' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING_ARRAY grid as parameter.
 */

// grid = ['ebacd', 'fghij', 'olmkn', 'trpqs', 'xywuv']

fn gridChallenge(grid: &[String]) -> String {
  let grid: Vec<Vec<u8>> = grid
    .iter()
    .map(|row| {
      let mut row = row.as_bytes().to_vec();
      row.sort();
      row
    })
    .collect();

  for col in 0..grid[0].len() {
    for row in 1..grid.len() {
      if grid[row][col] < grid[row - 1][col] {
        return "NO".to_string();
      }
    }
  }

  "YES".to_string()
}

fn main() {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

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

    let mut grid: Vec<String> = Vec::with_capacity(n as usize);

    for _ in 0..n {
      let grid_item = stdin_iterator.next().unwrap().unwrap();
      grid.push(grid_item);
    }

    let result = gridChallenge(&grid);

    writeln!(&mut fptr, "{}", result).ok();
  }
}
