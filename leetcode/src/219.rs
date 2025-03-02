use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::new();

    for (new_idx, num) in nums.iter().enumerate() {
      if let Some(old_idx) = map.insert(num, new_idx) {
        if (new_idx as i32) - (old_idx as i32) <= k {
          return true;
        }
      }
    }
    false
  }
}

fn main() {
  let nums = vec![1, 0, 1, 1];
  let k = 1;

  let expected = true;
  let result = Solution::contains_nearby_duplicate(nums, k);

  assert_eq!(expected, result);
}
