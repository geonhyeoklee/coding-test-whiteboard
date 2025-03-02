pub struct Solution;

impl Solution {
  pub fn reverse_bits(x: u32) -> u32 {
    let x = format!("{:b}", x);
    let count = 32 - x.len();

    let mut x: String = x.chars().rev().collect();

    for _ in 0..count {
      x.push('0');
    }

    u32::from_str_radix(&x, 2).unwrap()
  }
}

fn main() {
  let x = 43261596;
  let expected = 964176192;
  let result = Solution::reverse_bits(x);

  assert_eq!(expected, result);
}
