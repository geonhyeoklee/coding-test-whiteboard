fn palindrome_index(s: &str) -> isize {
  fn is_palindrome(s: &str, left: usize, right: usize) -> bool {
    let s_bytes = s.as_bytes();

    let mut l = left;
    let mut r = right;

    while l < r {
      if s_bytes[l] != s_bytes[r] {
        return false;
      }

      l += 1;

      // Prevent underflow
      if r == 0 {
        break;
      }

      r -= 1;
    }

    true
  }

  let mut left = 0;
  let mut right = s.len().saturating_sub(1);

  while left < right {
    let s_bytes = s.as_bytes();

    if s_bytes[left] != s_bytes[right] {
      if left + 1 <= right && is_palindrome(s, left + 1, right) {
        return left as isize;
      }

      if right > 0 && is_palindrome(s, left, right - 1) {
        return right as isize;
      }

      break;
    }

    left += 1;

    // Prevent underflow
    if right == 0 {
      break;
    }

    right -= 1;
  }

  -1
}

fn main() {
  let s = "abca";
  println!("{}", palindrome_index(s));
}
