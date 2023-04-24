use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: Api,
    pub trading: Trading,
    pub risk_management: RiskManagement,
}

#[derive(Debug, Deserialize)]
pub struct Api {
    pub key: String,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Trading {
    pub pairs: Vec<String>,
    pub trade_frequency: String,
    pub order_size: f64,
    pub risk_per_trade: f64,
}

#[derive(Debug, Deserialize)]
pub struct RiskManagement {
    pub stop_loss: f64,
    pub take_profit: f64,
}

impl Config {
    pub fn load(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let config = toml::from_str(&content)?;

        Ok(config)
    }
}