pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
        nums1.drain(m as usize.. );
        nums2.drain(n as usize..);

        nums1.append(nums2);
        nums1.sort();
        nums1.to_vec()
    }
}
