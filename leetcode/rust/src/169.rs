use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, i32>::new();

        for num in nums {
            if let Some(key) = map.get(&num) {
                map.insert(num, key + 1);
            } else {
                map.insert(num, 1);
            }
        }

        let mut max = (0 , 0);
        for (k, v) in map {
            if v > max.1 {
                max.0 = k;
                max.1 = v;
            }
        }

        max.0
    }
}

// fn main () {
//     let nums = vec![2,2,1,1,1,2,2];
//     let expected = 2;
//     let result = Solution::majority_element(nums);

//     assert_eq!(expected, result);
// }