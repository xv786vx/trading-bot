extern crate dotenv_codegen;
use dotenv::dotenv;
use std::env;


fn main() {
    
    dotenv().ok();

    let response = reqwest::blocking::get(
        "https://www.tradingview.com/markets/stocks-usa/market-movers-active/"
    )
    .unwrap()
    .text()
    .unwrap();

    let document = s craper::Html::parse_document(&response);

    let ticker_selector = scraper::Selector::parse("span.tickerCell-hMpTPJiS>a").unwrap();

    let tickers = document.select(&ticker_selector).map(|x| x.inner_html());
    tickers.zip(1..11).for_each(|(item, number)| println!("{}. {}", number, item));
    
    


    //sending request
    //let base_url = "https://api.twelvedata.com/time_series";
    requesting_ticker_data(String::from("AAPL")) //apparently you can't call async functions in another function
}

fn ticker_scrape(ticker_array: [String; 10]) {
    //scraper
    
}

async fn requesting_ticker_data(ticker: String) {
    dotenv().ok();
    
    let url = reqwest::get("https://api.twelvedata.com/time_series?symbol=".to_owned() + &ticker + "&interval=1min&format=CSV&apikey=27b2236e32484420a24a891d85d84676")
    .await?
    .text()
    .await?;
    println!("{:?}", url);
}
