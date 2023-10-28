pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut current_lowest_price = prices[0];
    let mut profit: i32 = 0;

    for index in 0..prices.len() {
        let current_price = prices[index];
        
        //If we can buy at a cheaper or same price as the current lowest price, we buy.
        if current_price <= current_lowest_price {
            current_lowest_price = current_price;
        }else {
            //If we can sell at a profit(higher price than the current lowest price), we sell.
            let current_profit = current_price - current_lowest_price;
            profit += current_profit;
            current_lowest_price = current_price;
        }
    }
    return profit;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_case_1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let output = max_profit(input);
        assert_eq!(output, 7);
    }

    #[test]
    fn test_case_2() {
        let input = vec![1, 2, 3, 4, 5];
        let output = max_profit(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn test_case_3() {
        let input = vec![7, 6, 4, 3, 1];
        let output = max_profit(input);
        assert_eq!(output, 0);
    }

}