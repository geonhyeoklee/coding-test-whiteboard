use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let mut char_table: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
      if char_table.contains_key(&c) {
        let current_value = char_table.get(&c).unwrap();
        char_table.insert(c, current_value + 1);
      } else {
        char_table.insert(c, 1);
      }
    }

    for c in t.chars() {
      if char_table.contains_key(&c) {
        let current_value = char_table.get(&c).unwrap();

        if *current_value > 0 {
          char_table.insert(c, current_value - 1);
        } else {
          return false;
        }
      } else {
        return false;
      }
    }

    true
  }
}

fn main() {
  let s = "anagram".to_string();
  let t = "nagaram".to_string();
  let expected = true;

  let result = Solution::is_anagram(s, t);

  assert_eq!(expected, result);
}
