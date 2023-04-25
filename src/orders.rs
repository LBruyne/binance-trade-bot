use binance::api::*;
use binance::account::*;

pub fn place_order(trading_pair: &str, order_type: &str, api_key: &str, secret_key: &str, dry_run: bool) {
    if dry_run {
        println!("Dry run: {} order would have been placed for {}", order_type, trading_pair);
    } else {
        let account: Account = Binance::new(Some(api_key.to_owned()), Some(secret_key.to_owned()));

        // Set the order size, symbol, and side according to the order_type
        let order_size = 0.01; // Replace this with your order size from the config
        let symbol = trading_pair;
        let side = match order_type {
            "Buy" => OrderSide::Buy,
            "Sell" => OrderSide::Sell,
            _ => panic!("Invalid order type"),
        };

        // Place a market order
        let result = account.custom_order(symbol, order_size, 0.0, None, side, OrderType::Market, TimeInForce::GTC, None);

        match result {
            Ok(order) => println!("Order placed: {:?}", order),
            Err(e) => println!("Error placing order: {:?}", e),
        }
    }
}
