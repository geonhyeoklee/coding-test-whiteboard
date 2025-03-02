struct Solution;
impl Solution {
  pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k = k as usize % n;

    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
  }
}
fn main() {
  let mut nums = vec![1, 2, 3];
  let k = 4;

  Solution::rotate(&mut nums, k);
}

// 두번째 솔루션
// struct Solution;
// impl Solution {
//     pub fn rotate(nums: &mut Vec<i32>, k: i32) {
//         let n = nums.len();
//         let k = k as usize % n;

//         nums.reverse();
//         nums[..k].reverse();
//         nums[k..].reverse();
//     }
// }
// fn main() {
//     let mut nums = vec![1, 2, 3];
//     let k = 4;

//     Solution::rotate(&mut nums, k);
// }
