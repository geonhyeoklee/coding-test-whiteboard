struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let (mut max, mut end, mut count) = (0, 0, 0);

        for idx in 0..len - 1 {
            max = max.max(idx + nums[idx] as usize);

            if idx == end {
                count += 1;
                end = max
            }
        }
        count as i32
    }
}

fn main() {
    let nums = vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3];
    let result = Solution::jump(nums);
    println!("{}", result);
}
