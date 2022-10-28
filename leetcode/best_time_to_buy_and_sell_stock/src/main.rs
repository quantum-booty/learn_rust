fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut left = 0;
        let mut right = 1;

        while right < prices.len() {
            if prices[left] < prices[right] {
                let profit = prices[right] - prices[left];
                max_profit = max_profit.max(profit);
            } else {
                left = right;
            }
            right += 1;
        }

        max_profit
    }
}
