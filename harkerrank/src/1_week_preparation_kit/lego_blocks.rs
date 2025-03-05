use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'legoBlocks' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER m
 */

// Problem
// 2 2     n = 2, m = 2
// 3 2     n = 3, m = 2
// 2 3     n = 2, m = 3
// 4 4     n = 4, m = 4

// Result
// 3
// 7
// 9
// 3375
fn legoBlocks(n: i32, m: i32) -> i32 {
  const MOD: i64 = 1_000_000_007;
  let (n, m) = (n as usize, m as usize);

  // Calculate combinations for a single row
  let mut row = vec![0i64; m + 1]; // 한 행에 대한 가능한 조합을 저장할 벡터
  row[0] = 1; // 기본 케이스

  // 너비 j에 대해, 1~4 크기의 블록을 사용하여 만들 수 있는 모든 조합 계산
  for j in 1..=m {
    for k in 1..=4 {
      if j >= k {
        row[j] = (row[j] + row[j - k]) % MOD;
      }
    }
  }

  // Calculate total combinations for n rows
  let mut total = vec![0i64; m + 1];
  for j in 1..=m {
    total[j] = row[j];
    for _ in 1..n {
      total[j] = (total[j] * row[j]) % MOD;
    }
  }

  // Calculate valid combinations (without horizontal cracks)
  let mut valid = vec![0i64; m + 1];
  valid[1] = total[1];

  for j in 2..=m {
    let mut sum = total[j];
    for k in 1..j {
      let invalid = (valid[k] * total[j - k]) % MOD;
      sum = (sum - invalid + MOD) % MOD;
    }
    valid[j] = sum;
  }

  valid[m] as i32
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
    let first_multiple_input: Vec<String> = stdin_iterator
      .next()
      .unwrap()
      .unwrap()
      .split(' ')
      .map(|s| s.to_string())
      .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = legoBlocks(n, m);

    writeln!(&mut fptr, "{}", result).ok();
  }
}
