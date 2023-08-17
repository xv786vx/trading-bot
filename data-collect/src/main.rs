mod config;
mod fetcher;
mod preprocessing;

use crate::preprocessing::parse_merged_csv;

use fetcher::Fetcher;
use std::error::Error;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut fetcher: Fetcher = Fetcher::new(vec![
       "SPY", "VIX", "rsi", "atr", "ema_9", "ema_12", "ema_26", "macd", "vwap",
    ]);

    fetcher.get_data_for_nn(3, "30min").await;
    fetcher.merge_csvs();
    parse_merged_csv(true)?;

    Ok(())
}
