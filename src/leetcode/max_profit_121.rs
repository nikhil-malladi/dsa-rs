use std::cmp;

pub fn max_profit_1(prices: Vec<i32>) -> i32 {
    let mut min_price = std::i32::MAX;
    let mut max_profit = 0;
    for price in prices {
        min_price = cmp::min(min_price, price);
        max_profit = cmp::max(max_profit, price - min_price);
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_max_profit() {
        let vec: Vec<i32> = vec![7,1,5,3,6,4];
        let test_result: i32 = max_profit_1(vec);
        const ideal_result: i32 = 5;
        assert_eq!(test_result,ideal_result);
    }

    #[test]
    fn test_2_max_profit() {
        let vec: Vec<i32> = vec![3,5,1,2,2,8];
        let test_result: i32 = max_profit_1(vec);
        const ideal_result: i32 = 7;
        assert_eq!(test_result,ideal_result);
    }
}
