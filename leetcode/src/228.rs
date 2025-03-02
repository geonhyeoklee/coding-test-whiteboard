pub struct Solution;

impl Solution {
  pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
      return vec![];
    }

    let mut start = nums[0];
    let mut ranges: Vec<String> = Vec::with_capacity(nums.len());

    let mut peekable_nums = nums.iter().peekable();

    for _ in 0..nums.len() {
      let next = *peekable_nums.next().unwrap();
      let maybe_peek = peekable_nums.peek();

      match maybe_peek {
        Some(peek) => {
          if next + 1 == **peek {
            continue;
          } else {
            ranges.push(Self::create_range(start, next));
            start = **peek;
          }
        }
        None => {
          ranges.push(Self::create_range(start, next));
          break;
        }
      }
    }

    ranges
  }

  fn create_range(start: i32, end: i32) -> String {
    if start == end {
      format!("{}", end)
    } else {
      format!("{}->{}", start, end)
    }
  }
}

fn main() {
  let nums = vec![0, 1, 2, 4, 5, 7];

  let result = Solution::summary_ranges(nums);

  println!("{:?}", result);
}
