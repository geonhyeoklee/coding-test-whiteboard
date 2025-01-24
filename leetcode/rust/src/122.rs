struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit: i32 = 0;

        for idx in 0..prices.len() - 1 {
            let cur = prices.get(idx).unwrap();
            let peek = prices.get(idx + 1).unwrap();

            if peek - cur > 0 {
                max_profit += peek - cur;
            }
        }

        max_profit
    }
}
fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::max_profit(prices);
    println!("{}", result);
}
