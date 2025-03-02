struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut furthest_reachable = 0;

        for i in 0..len - 1 {
            furthest_reachable = furthest_reachable.max(i + nums[i] as usize);
            if furthest_reachable == i {
                return false;
            }
        }
        true
    }
}

fn main() {
    let nums = vec![2, 0];
    let result = Solution::can_jump(nums);
    println!("{}", result);
}
