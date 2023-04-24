pub struct Kline {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_asset_volume: f64,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}

impl From<&binance::model::KlineSummary> for Kline {
    fn from(summary: &binance::model::KlineSummary) -> Self {
        Kline {
            open_time: summary.open_time,
            open: summary.open.parse().unwrap(),
            high: summary.high.parse().unwrap(),
            low: summary.low.parse().unwrap(),
            close: summary.close.parse().unwrap(),
            volume: summary.volume.parse().unwrap(),
            close_time: summary.close_time,
            quote_asset_volume: summary.quote_asset_volume.parse().unwrap(),
            number_of_trades: summary.number_of_trades,
            taker_buy_base_asset_volume: summary.taker_buy_base_asset_volume.parse().unwrap(),
            taker_buy_quote_asset_volume: summary.taker_buy_quote_asset_volume.parse().unwrap(),
        }
    }
}
