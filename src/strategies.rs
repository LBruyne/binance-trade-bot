use crate::kline::{Kline};
use crate::processor::{calculate_sma};

pub fn dual_sma_strategy(klines: &[Kline], short_period: usize, long_period: usize) -> Option<String> {
    if klines.len() < long_period {
        return None;
    }

    let short_sma = calculate_sma(&klines[(klines.len() - short_period)..], short_period);
    let long_sma = calculate_sma(&klines[(klines.len() - long_period)..], long_period);

    if !short_sma.is_empty() && !long_sma.is_empty() {
        let short_sma = short_sma[0];
        let long_sma = long_sma[0];

        if short_sma > long_sma {
            Some(String::from("Buy"))
        } else if short_sma < long_sma {
            Some(String::from("Sell"))
        } else {
            None
        }
    } else {
        None
    }
}
