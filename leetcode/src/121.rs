pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut profit,mut buy): (i32,i32) = (0, prices[0]);

        for i in 1..prices.len() {
            if prices[i] > buy {
                profit = profit.max(prices[i] - buy);
            } else {
                buy = buy.min(prices[i]);
            }
        }

        profit
    }
}

// fn main () {
//     // let nums = vec![7,1,5,3,6,4];
//     let nums = vec![7,2,4,8,1,6,9];
//     let expected = 6;
//     let result = Solution::max_profit(nums);

//     assert_eq!(expected, result);
// }