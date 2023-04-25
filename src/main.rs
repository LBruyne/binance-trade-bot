mod config;
mod kline;
mod processor;
mod strategies;
mod orders;

use binance::api::*;
use binance::market::*;
use std::time::{Instant, Duration};
use std::thread::sleep;
use crate::kline::Kline;
use crate::processor::process_klines;

fn main() {
    let config_file = "config.toml";
    let config = config::Config::load(config_file)
        .expect("Fail to load configuration file");

    println!("{:?}", config);

    let api_key = config.api.key;
    let secret_key = config.api.secret;

    let market: Market = Binance::new(Some(api_key.clone()), Some(secret_key.clone()));

    // Set the execution interval in seconds (e.g., 60 seconds)
    let interval = Duration::from_secs(60);

    loop {
        // Record the start time of the loop iteration
        let start = Instant::now();

        // Iterate through the trading pairs from the config
        for trading_pair in &config.trading.pairs {
            // Fetch and preprocess historical data for the trading pair and interval
            let interval = &config.trading.trade_frequency;
            let limit = 500;

            let klines = market
                .get_klines(trading_pair, interval, limit, None, None)
                .expect("Failed to fetch historical data");

            match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines_data) => {
                    // let klines0: KlineSummary = klines_data[0].clone();
                    // // println!(
                    // //     "Open: {}, High: {}, Low: {}",
                    // //     klines0.open, klines0.high, klines0.low
                    // // );

                    let klines_data: Vec<Kline> = klines_data.iter().map(|kline| Kline::from(kline)).collect();
                    process_klines(&klines_data, trading_pair, &api_key, &secret_key, false);
                }
            }
        }

        // Calculate the elapsed time since the start of the loop iteration
        let elapsed = start.elapsed();

        // Sleep for the remaining duration of the interval, if any
        if elapsed < interval {
            sleep(interval - elapsed);
        }
    }
}