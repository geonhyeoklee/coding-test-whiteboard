struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut slow, mut fast) = (0 as usize, 1 as usize);

        while fast < nums.len(){
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            }

            fast += 1;
        }

        slow as i32 + 1
    }
}
// fn main () {
//     let mut nums = vec![0,1,1,2,2,3,4];
//     let expected = 5;
//     let len = Solution::remove_duplicates(&mut nums);

//     assert_eq!(expected, len);
// }