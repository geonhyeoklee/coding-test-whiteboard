use std::collections::HashMap;

pub struct Solution;
impl Solution {
  pub fn is_isomorphic(s: String, t: String) -> bool {
    assert_eq!(s.len(), t.len());

    let mut table: HashMap<char, char> = HashMap::new();
    let mut values: Vec<char> = Vec::new();

    for (s, t) in s.chars().zip(t.chars()) {
      match table.insert(s, t) {
        Some(v) if v == t => continue,
        Some(_) => return false,
        None => (),
      };

      if values.contains(&t) {
        return false;
      }

      values.push(t);
    }

    true
  }
}

fn main() {
  let a = "foo".to_string();
  let b = "bar".to_string();

  let expected = false;
  let result = Solution::is_isomorphic(a, b);

  assert_eq!(expected, result);
}
