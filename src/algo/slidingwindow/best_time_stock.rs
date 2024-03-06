use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_buy = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            if min_buy > price {
                min_buy = price;
                continue;
            }

            max_profit = max_profit.max(price - min_buy);
        }

        max_profit
    }
}

#[cfg(test)]
mod test_best_time_stock {
    use crate::algo::slidingwindow::Solution;

    #[test]
    fn test1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 4, 5, 6, 3]))
    }

    #[test]
    fn test2() {
        assert_eq!(2, Solution::max_profit(vec![2, 4, 1]))
    }
}
