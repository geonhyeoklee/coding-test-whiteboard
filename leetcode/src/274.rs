struct Solution;
impl Solution {
  pub fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort();
    citations.reverse();
    let mut h_index = 0;

    for (idx, citation) in citations.into_iter().enumerate() {
      let may_h_index = (idx + 1) as i32;

      if citation >= may_h_index {
        h_index = may_h_index;
      }
    }

    h_index
  }
}

fn main() {
  let nums = vec![3, 0, 6, 1, 5];
  let result = Solution::h_index(nums);
  println!("{}", result);
}
