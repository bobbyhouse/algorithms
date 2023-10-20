fn main() {
    let stock_prices = vec![8, 6, 8, 8, 4, 4, 9, 14, 2, 8];
    let max_profit = max_stock_profit(stock_prices);
    println!("Max profit: {}", max_profit);
}

fn max_stock_profit(prices: Vec<u32>) -> u32 {
    let mut max_profit = 0;
    let mut min_price = prices[0];

    for price in prices {
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }

    return max_profit;
}

//fn max_stock_profit(prices: &vec) -> u32 {
//    return prices[0];
//}
