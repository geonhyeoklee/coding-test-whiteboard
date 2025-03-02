struct Solution;

impl Solution {
  pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut result = usize::MAX;
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;

    while end < nums.len() {
      sum += nums[end];
      end += 1;

      while sum >= target {
        if sum - nums[start] >= target {
          sum -= nums[start];
          start += 1;
        } else {
          result = result.min(start - end); // Length
          break;
        }
      }
    }

    if result == usize::MAX {
      result = 0;
    }

    result as i32
  }
}

fn main() {
  let target = 7;
  let nums = vec![2, 3, 1, 2, 4, 3];
  let result = Solution::min_sub_array_len(target, nums);
  println!("Result: {:?}", result);
}
