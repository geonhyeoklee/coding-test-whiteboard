struct Solution;
// Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
impl Solution {
  pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut stack = vec![];
    stack.push((0, vec![]));

    while let Some((index, mut current)) = stack.pop() {
      if index == nums.len() {
        result.push(current);
        continue;
      }

      stack.push((index + 1, current.clone()));
      current.push(nums[index]);
      stack.push((index + 1, current));
    }

    result.sort_by(|a, b| a.len().cmp(&b.len()));
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_subsets_example1() {
    let nums = vec![1, 2, 3];
    let result = Solution::subsets(nums);
    let expected = vec![
      vec![],
      vec![1],
      vec![2],
      vec![3],
      vec![1, 2],
      vec![1, 3],
      vec![2, 3],
      vec![1, 2, 3],
    ];
    assert_eq!(result, expected);
  }

  #[test]
  fn test_subsets_example2() {
    let nums = vec![0];
    let result = Solution::subsets(nums);
    let expected = vec![vec![], vec![0]];
    assert_eq!(result, expected);
  }
}

fn main() {}
