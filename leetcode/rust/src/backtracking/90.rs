struct Solution;

impl Solution {
  pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut result = vec![vec![]];

    let mut stack = vec![];
    stack.push((0, vec![]));

    while let Some((start, mut current)) = stack.pop() {
      for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
          continue;
        }

        current.push(nums[i]);
        result.push(current.clone());
        stack.push((i + 1, current.clone()));
        current.pop();
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
  fn test_subsets_with_dup() {
    let test_cases = vec![
      (
        vec![1, 2, 2],
        vec![
          vec![],
          vec![1],
          vec![1, 2],
          vec![1, 2, 2],
          vec![2],
          vec![2, 2],
        ],
      ),
      (vec![0], vec![vec![], vec![0]]),
      (
        vec![4, 4, 4, 1, 4],
        vec![
          vec![],
          vec![1],
          vec![1, 4],
          vec![1, 4, 4],
          vec![1, 4, 4, 4],
          vec![1, 4, 4, 4, 4],
          vec![4],
          vec![4, 4],
          vec![4, 4, 4],
          vec![4, 4, 4, 4],
        ],
      ),
    ];

    for (nums, expected) in test_cases {
      let result = Solution::subsets_with_dup(nums);
      assert_eq!(result, expected);
    }
  }
}

fn main() {}
