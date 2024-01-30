mod config;
mod fetcher;
mod preprocessing;

use crate::preprocessing::parse_merged_csv;

use fetcher::Fetcher;
use std::{
    error::Error,
    io::{prelude::*, stdin, stdout},
};
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut fetcher: Fetcher = Fetcher::new(vec![
        "SPY", "VIX", "rsi", "atr", "ema_9", "ema_12", "ema_26", "macd", "vwap",
    ]);

    let debug: bool = yn("Debug mode? [y/N]\n> ");

    fetcher.get_data_for_nn(2, "1day", debug).await;
    fetcher.merge_csvs("1day");

    parse_merged_csv(yn("Normalize data? [y/N]\n> "))?;

    Ok(())
}

fn yn(prompt: &str) -> bool {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input: String = input.trim().to_lowercase();

    match trimmed_input.as_str() {
        "y" => true,
        "n" => false,
        _ => false,
    }
}
