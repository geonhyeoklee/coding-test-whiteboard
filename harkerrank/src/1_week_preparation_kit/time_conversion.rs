use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

// 07:05:45PM
// 19:05:45

fn timeConversion(s: &str) -> String {
  let is_pm = s.ends_with("PM");
  let mut s = s.split(':');

  let mut hour = s.nth(0).unwrap().parse::<i32>().unwrap();
  let minute = s.nth(1).unwrap().parse::<i32>().unwrap();
  let second = s.nth(2).unwrap().parse::<i32>().unwrap();

  match is_pm {
    true => {
      if hour < 12 {
        hour += 12;
      }
    }
    false => {
      if hour >= 12 {
        hour -= 12;
      }
    }
  }

  hour.to_string() + ":" + &minute.to_string() + ":" + &second.to_string()
}

fn main() {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

  let s = stdin_iterator.next().unwrap().unwrap();

  let result = timeConversion(&s);

  writeln!(&mut fptr, "{}", result).ok();
}
