mod config;
mod fetcher;

use fetcher::Fetcher;
use reqwest::Error;
use tokio::main;

#[main]
async fn main() -> Result<(), Error> {
    let assets: Vec<&str> = vec![
        "SPY", "VIX", "rsi", "atr", "ema_9", "ema_12", "ema_26", "macd", "vwap",
    ];

    let mut fetcher: Fetcher = Fetcher::new(assets);

    fetcher.get_data_for_nn(3).await;
    fetcher.merge_csvs();
    Ok(())
}
