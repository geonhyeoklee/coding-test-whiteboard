use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();

        let ref_nums = &nums[..];

        let mut res: HashSet<Vec<i32>> = HashSet::new();

        for i in 0..nums.len() - 2 {
            let n1 = ref_nums[i];

            let mut p0 = i + 1;
            let mut p1 = nums.len() - 1;

            while p0 < p1 {
                let n2 = ref_nums[p0];
                let n3 = ref_nums[p1];

                let sum = n1 + n2 + n3;

                if sum < 0 {
                    p0 += 1;
                } else if sum > 0 {
                    p1 -= 1;
                } else {
                    res.insert(vec![n1, n2, n3]);

                    while p0 < p1 && ref_nums[p0] == n2 {
                        p0 += 1;
                    }

                    while p0 < p1 && ref_nums[p1] == n3 {
                        p1 -= 1;
                    }
                }
            }
        }

        res.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];

    let expected: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    let result = Solution::three_sum(nums);

    assert_eq!(expected, result);
}
