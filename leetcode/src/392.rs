pub struct Solution;

impl Solution {
  pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 {
      return true;
    }

    let t_bytes = t.into_bytes();
    let s_bytes = s.into_bytes();

    let mut p = 0;

    for t_byte in t_bytes {
      if s_bytes[p] == t_byte {
        p += 1;
      }

      if p >= s_bytes.len() {
        return true;
      }
    }

    false
  }
}
fn main() {
  let s = "abc".to_string();
  let t = "ahbgdc".to_string();

  let expected = true;
  let result = Solution::is_subsequence(s, t);

  assert_eq!(expected, result);
}
