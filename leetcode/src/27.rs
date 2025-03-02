pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut p1 : i32 = 0;
        let mut p2 = (nums.len() - 1) as i32;

        while p1 < nums.len() as i32 && p2 > -1 && p1 <= p2{
            if nums[p1 as usize] == val {
                nums[p1 as usize] = nums[p2 as usize];
                p2 -= 1;
            } else {
                p1 += 1;
            }
        }

        println!("{:?} {:?}", p1, p2);
        println!("{:?}", nums);

        p1
    }
}