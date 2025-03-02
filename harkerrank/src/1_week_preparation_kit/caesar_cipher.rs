use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'caesarCipher' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. INTEGER k
 */

fn caesarCipher(s: &str, k: i32) -> String {
  let original = "abcdefghijklmnopqrstuvwxyz";
  let rotated = original
    .chars()
    .cycle()
    .skip(k as usize)
    .take(original.len())
    .collect::<String>();

  let mut map = std::collections::HashMap::new();
  for (o, r) in original.chars().zip(rotated.chars()) {
    map.insert(o, r);
  }

  s.chars()
    .map(|c| {
      if c.is_ascii_alphabetic() {
        if c.is_ascii_lowercase() {
          map.get(&c).unwrap().to_owned()
        } else {
          map
            .get(&c.to_ascii_lowercase())
            .unwrap()
            .to_ascii_uppercase()
        }
      } else {
        c
      }
    })
    .collect::<String>()
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

  let s = stdin_iterator.next().unwrap().unwrap();

  let k = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim()
    .parse::<i32>()
    .unwrap();

  let result = caesarCipher(&s, k);

  writeln!(&mut fptr, "{}", result).ok();
}
