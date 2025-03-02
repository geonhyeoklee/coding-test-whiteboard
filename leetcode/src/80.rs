struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut idx: usize = 2;

        while idx <= len - 1 {
            match Self::check_duplicate(nums.to_vec(), idx) {
                true => {
                    nums.remove(idx);
                    len -= 1;
                }
                false => {
                    idx += 1;
                }
            }
        }

        nums.len() as i32
    }

    fn check_duplicate(nums: Vec<i32>, idx: usize) -> bool {
        nums[idx] == nums[idx - 1] && nums[idx] == nums[idx - 2]
    }
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];

    let result = Solution::remove_duplicates(&mut nums);
    println!("{}", result);
}
