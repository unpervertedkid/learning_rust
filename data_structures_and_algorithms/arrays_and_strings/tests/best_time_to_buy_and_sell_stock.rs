use std::cmp::max;
use std::cmp::min;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut lowest_price_so_far = prices[0];
    let mut profit: i32 = 0;

    for index in 0..prices.len() {
        let current_price = prices[index];
        lowest_price_so_far = min(current_price, lowest_price_so_far);

        let current_profit = current_price - lowest_price_so_far;
        profit = max(profit, current_profit);
    }

    return profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let output = max_profit(input);
        assert_eq!(output, 5);
    }

    #[test]
    fn test_case_2() {
        let input = vec![7, 6, 4, 3, 1];
        let output = max_profit(input);
        assert_eq!(output, 0);
    }
}
