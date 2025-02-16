pub fn search(nums: Vec<i32>, target: i32) -> i32 {
  let mut stack = vec![];
  stack.push((0, nums.len().saturating_sub(1)));

  while let Some((left, right)) = stack.pop() {
    if left > right {
      continue;
    }

    let mid = (left + right) / 2;

    if nums[mid] == target {
      return mid as i32;
    }

    if nums[left] <= nums[mid] { // left is sorted
      if nums[left] <= target && target <= nums[mid] { // target in left
        stack.push((left, mid - 1));
      } else { // target in right
        stack.push((mid + 1, right));
      }
      continue;
    }

    if nums[mid] < nums[right] { // right is sorted
      if nums[mid] <= target && target <= nums[right] { // target in right
        stack.push((mid + 1, right));
      } else { // target in left
        stack.push((left, mid - 1));
      }
    }
  }

  -1
}

// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
    }
}


fn main() {}