use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total_profit = 0;
        let mut i = 0;

        while i < prices.len() {
            let mut buy_price = 0;
            if prices[i + 1] > prices[i] {
                // buy the stock
                println!("Day {i}: bought at {}", prices[i]);
                buy_price = prices[i];
                i += 1;
                while i < prices.len() {
                    if prices[i + 1] < prices[i] {
                        // sell the stock
                        println!("Day {i}: sold at {}", prices[i]);
                        total_profit += prices[i] - buy_price;
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
            i += 1;
        }

        total_profit
    }
}

fn main() {
    println!("Hello, world!");
}
