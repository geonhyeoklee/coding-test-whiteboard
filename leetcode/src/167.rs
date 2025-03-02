pub struct Solution;

impl Solution {
  pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut p0 = 0;
    let mut p1 = numbers.len() - 1;

    while p0 < p1 {
      let sum = numbers[p0] + numbers[p1];

      if sum > target {
        p1 -= 1;
      } else if sum < target {
        p0 += 1;
      } else {
        result.push(p0 as i32 + 1);
        result.push(p1 as i32 + 1);
        break;
      }
    }

    result
  }
}

fn main() {
  let numbers = vec![2, 7, 11, 15];
  let target = 9;

  let expected = vec![1, 2];
  let result = Solution::two_sum(numbers, target);

  assert_eq!(expected, result);
}
