pub struct Solution;

impl Solution {
  pub fn is_happy(n: i32) -> bool {
    if n == 1 || n == 7 {
      return true;
    }

    if n < 10 {
      return false;
    }

    let sum_of_squares = Solution::sum_of_squares(n);

    if sum_of_squares.eq(&1) {
      return true;
    } else {
      return Solution::is_happy(sum_of_squares);
    }
  }

  fn sum_of_squares(n: i32) -> i32 {
    let binding = n.to_string();
    let chars = binding.chars();
    let map: Vec<i32> = chars
      .map(|v| v.to_digit(10).unwrap() as i32)
      .map(|v| v * v)
      .collect();
    let result = map.into_iter().fold(0, |acc, v| acc + v);
    result
  }
}

fn main() {
  let n = 19;
  let expected = true;
  let result = Solution::is_happy(n);

  assert_eq!(expected, result);
}
