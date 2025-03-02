use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn word_pattern(pattern: String, s: String) -> bool {
    let s: Vec<&str> = s.split(' ').collect();

    if pattern.len() != s.len() {
      return false;
    }

    let mut table: HashMap<&str, char> = HashMap::new();
    let mut used_pattern: Vec<char> = vec![];

    for (i, c) in pattern.chars().enumerate() {
      if !used_pattern.contains(&c) {
        table.insert(s[i], c);
        used_pattern.push(c);
      }
    }

    let s: String = s
      .into_iter()
      .map(|v| table.get(v).unwrap_or(&'_').to_owned())
      .collect();

    pattern.eq(&s)
  }
}

fn main() {
  let pattern = "jquery".to_string();
  let s = "jquery".to_string();
  let expected = false;

  let result = Solution::word_pattern(pattern, s);

  assert_eq!(expected, result);
}
