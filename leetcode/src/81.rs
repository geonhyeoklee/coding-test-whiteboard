pub fn search(nums: Vec<i32>, target: i32) -> bool {
  let mut left = 0;
  let mut right = nums.len().saturating_sub(1);

  while left <= right {
    let mid = (left + right) / 2;

    if nums[mid] == target {
      return true;
    }

    // Skip duplicates more efficiently
    while left < mid && nums[left] == nums[mid] {
      left += 1;
    }
    while right > mid && nums[right] == nums[mid] {
      right = right.saturating_sub(1);
    }

    if nums[left] <= nums[mid] { // left is sorted
      if nums[left] <= target && target < nums[mid] {
        right = mid.saturating_sub(1);
      } else {
        left = mid + 1;
      }

      continue;
    }

    if nums[mid] < nums[right] { // right is sorted
      if nums[mid] < target && target <= nums[right] {
        left = mid + 1;
      } else {
        right = mid.saturating_sub(1);
      }
    }
  }

  false
}

// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search(vec![2,5,6,0,0,1,2], 3), false);
        assert_eq!(search(vec![2,5,6,0,0,1,2], 0), true);
        assert_eq!(search(vec![1,0,1,1,1], 0), true);
    }

    #[test]
    fn timelimit_test() {
        assert_eq!(search(vec![1,1,1,1,1,1,1,1,1,13,1,1,1,1,1,1,1,1,1,1,1,1], 13), true);
    }
}


fn main() {}