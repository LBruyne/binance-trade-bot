use std::collections::VecDeque;
use crate::Kline;
use crate::strategies::dual_sma_strategy;
use crate::orders::place_order;

pub fn process_klines(klines: &[Kline], trading_pair: &str, api_key: &str, secret_key: &str, dry_run: bool) {
    let signal = dual_sma_strategy(&klines, 10, 50);
    if let Some(signal) = signal {
        println!("Signal: {}", signal);
        place_order(trading_pair, &signal, api_key, secret_key, dry_run);
    }
}

/// This function calculates the Simple Moving Average (SMA) of the given price data.
///
/// The SMA is a popular technical indicator that helps in smoothing out price fluctuations
/// and identifying trends in financial markets. It calculates the average of the closing
/// prices over a specified period, and as new data points are added, the oldest data points
/// are removed, keeping the average up-to-date.
pub fn calculate_sma(klines_data: &[Kline], window: usize) -> Vec<f64> {
    let mut sma = vec![];
    let mut sum = 0.0;
    let mut queue = VecDeque::new();

    for kline in klines_data {
        queue.push_back(kline.close);
        sum += kline.close;

        if queue.len() > window {
            sum -= queue.pop_front().unwrap();
        }

        if queue.len() == window {
            sma.push(sum / window as f64);
        }
    }

    sma
}
