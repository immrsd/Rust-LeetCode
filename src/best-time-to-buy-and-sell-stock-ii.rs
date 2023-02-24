// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/solutions/3224039/rust-one-liner/

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).fold(0, |res, w| res + max(0, w[1] - w[0]))
    }
}
