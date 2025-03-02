struct Solution;
// Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
impl Solution {
  pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    let mut stack = vec![];
    stack.push((vec![], 1));

    while let Some((combined, start)) = stack.pop() {
      if combined.len() == k as usize {
        result.push(combined);
        continue;
      }

      for i in start..=n {
        let mut new_combined = combined.clone();
        new_combined.push(i);
        stack.push((new_combined, i + 1));
      }
    }

    result.sort();
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_combine() {
    let result = Solution::combine(4, 2);
    let expected = vec![
      vec![1, 2],
      vec![1, 3],
      vec![1, 4],
      vec![2, 3],
      vec![2, 4],
      vec![3, 4],
    ];
    assert_eq!(result, expected);
  }
}

fn main() {
  println!("Hello, world!");
}
