use std::collections::HashMap;

pub struct Solution;
impl Solution {
  pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut magazine_map: HashMap<char, usize> = HashMap::new();

    let chars = magazine.chars().into_iter();
    for c in chars {
      if magazine_map.contains_key(&c) {
        if let Some(count) = magazine_map.get(&c) {
          magazine_map.insert(c, count.clone() + 1);
        }
      } else {
        magazine_map.insert(c, 1);
      }
    }

    let chars = ransom_note.chars().into_iter();
    for c in chars {
      if magazine_map.contains_key(&c) {
        if let Some(count) = magazine_map.get(&c) {
          if *count > 0 {
            magazine_map.insert(c, count.clone() - 1);
          } else {
            return false;
          }
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
  let a = "aabbc".to_string();
  let b = "aabbbc".to_string();

  let expected = true;
  let result = Solution::can_construct(a, b);

  assert_eq!(expected, result);
}
