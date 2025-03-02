struct Solution;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut map: HashMap<char, usize> = HashMap::new();
    let mut left: i32 = -1;
    let mut max_len = 0;

    for (right, _char) in s.chars().enumerate() {
      match map.get(&_char) {
        Some(idx) => {
          left = left.max(*idx as i32);
        }
        None => (),
      };

      max_len = max(max_len, right as i32 - left);
      map.insert(_char, right);
    }

    max_len
  }
}

fn main() {
  let s = "au".to_string();
  let result = Solution::length_of_longest_substring(s);
  println!("Result: {:?}", result);
}
