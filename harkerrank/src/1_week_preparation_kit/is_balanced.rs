use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'isBalanced' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn isBalanced(s: &str) -> String {
  let mut stack = Vec::new();

  for c in s.chars() {
    match c {
      '(' | '[' | '{' => stack.push(c),
      ')' => {
        if let Some(&'(') = stack.last() {
          stack.pop();
        } else {
          return "NO".to_string();
        }
      }
      ']' => {
        if let Some(&'[') = stack.last() {
          stack.pop();
        } else {
          return "NO".to_string();
        }
      }
      '}' => {
        if let Some(&'{') = stack.last() {
          stack.pop();
        } else {
          return "NO".to_string();
        }
      }
      _ => (),
    }
  }

  if stack.is_empty() {
    return "YES".to_string();
  } else {
    return "NO".to_string();
  }
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
    let s = stdin_iterator.next().unwrap().unwrap();

    let result = isBalanced(&s);

    writeln!(&mut fptr, "{}", result).ok();
  }
}
