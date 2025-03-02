pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut p0 = 0;
        let mut p1 = height.len() - 1;
        let mut area = 0;

        while p0 < p1 {
            let w = p1 - p0;

            let is_higher_p0 = height[p0] > height[p1];

            let h = if is_higher_p0 { height[p1] } else { height[p0] };

            if w as i32 * h > area {
                area = w as i32 * h;
            }

            if is_higher_p0 {
                p1 -= 1;
            } else {
                p0 += 1;
            }
        }

        area
    }
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

    let expected = 49;
    let result = Solution::max_area(height);

    assert_eq!(expected, result);
}
