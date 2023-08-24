use crate::config::{BASE_URL, KEYS, NUM_KEYS};
use chrono::{prelude::*, ParseError};
use csv::{Reader, ReaderBuilder, StringRecord, Writer};
use reqwest::{Client, Error, Request};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{create_dir_all, read_dir, remove_file, DirEntry, File, OpenOptions},
    io::{stdout, Write},
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

pub enum KeyStatus {
    AllKeysFailed,
    Success,
}

impl Display for KeyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyStatus::Success => write!(f, "success"),
            KeyStatus::AllKeysFailed => write!(f, "failure"),
        }
    }
}

pub struct Fetcher {
    pub base_url: &'static str,
    pub keys: [&'static str; NUM_KEYS],
    pub ratelimited_keys: Vec<&'static str>,
    pub assets: Vec<&'static str>,
    client: Client,
}

impl Fetcher {
    pub fn new(assets: Vec<&'static str>) -> Self {
        print!("Clearing old data...");
        stdout().flush().expect("Failed to flush stdout");

        let data_path: &Path = Path::new("data");
        create_dir_all(data_path).expect("Failed to create data directory");

        for entry in read_dir(data_path).expect("Failed to read data folder contents") {
            let entry: DirEntry = entry.expect("Failed to get folder entry");
            remove_file(entry.path()).expect("Failed to delete entry");
        }
        println!("done");

        Fetcher {
            base_url: BASE_URL,
            keys: KEYS,
            ratelimited_keys: Vec::new(),
            assets,
            client: Client::new(),
        }
    }

    async fn request(
        &mut self,
        route: &str,
        params: Vec<(&str, &str)>,
    ) -> Result<(Value, KeyStatus), Error> {
        let mut response_parent: Value = Value::Null;
        let mut key_status: KeyStatus = KeyStatus::AllKeysFailed;

        for key in self.keys.iter() {
            if self.ratelimited_keys.contains(key) {
                continue;
            }

            let request: Request = self
                .client
                .get(format!("{}/{}", self.base_url, route))
                .header("Authorization", format!("apikey {}", key))
                .query(&params)
                .build()
                .expect("Failed to build request");

            let response: Value = match match self.client.execute(request).await {
                Ok(response) => response,
                Err(err) => return Err(err),
            }
            .json::<Value>()
            .await
            {
                Ok(value) => value,
                Err(err) => return Err(err),
            };

            if response["status"] == "ok" {
                response_parent = response;
                key_status = KeyStatus::Success;
                break;
            } else if response["status"] == "error"
                && !response["code"].is_null()
                && response["code"] == 429
            {
                self.ratelimited_keys.push(key);
                continue;
            }
        }

        Ok((response_parent, key_status))
    }

    pub async fn get_data(
        &mut self,
        symbol: &str,
        date: &str,
        timeframe: &str,
    ) -> Result<Value, Error> {
        let mut valid: bool = false;
        let mut data_parent: Value = Value::Null;

        while !valid {
            match symbol {
                "SPY" | "VIX" => {
                    let (data, key_status): (Value, KeyStatus) = match self
                        .request(
                            "time_series",
                            vec![
                                ("symbol", symbol),
                                ("format", "JSON"),
                                ("interval", timeframe),
                                ("outputsize", "5000"),
                                ("end_date", date),
                            ],
                        )
                        .await
                    {
                        Ok(data) => data,
                        Err(err) => return Err(err),
                    };

                    match key_status {
                        KeyStatus::Success => {
                            valid = true;
                            data_parent = data;
                        }
                        KeyStatus::AllKeysFailed => {
                            println!("All keys failed! Cooldown...");
                            sleep(Duration::from_secs(60));
                            self.ratelimited_keys = Vec::new();
                            println!("Retrying...");
                        }
                    }
                }

                "ema_9" | "ema_12" | "ema_26" => {
                    let mut time_period: i32 = 0;
                    if let Some(index) = symbol.rfind('_') {
                        if let Ok(number) = symbol[index + 1..].parse::<i32>() {
                            time_period = number;
                        }
                    }
                    let (data, key_status): (Value, KeyStatus) = match self
                        .request(
                            "ema",
                            vec![
                                ("symbol", "SPY"),
                                ("format", "JSON"),
                                ("interval", timeframe),
                                ("outputsize", "5000"),
                                ("time_period", &time_period.to_string()),
                                ("end_date", date),
                            ],
                        )
                        .await
                    {
                        Ok(data) => data,
                        Err(err) => return Err(err),
                    };

                    match key_status {
                        KeyStatus::Success => {
                            valid = true;
                            data_parent = data;
                        }
                        KeyStatus::AllKeysFailed => {
                            println!("All keys failed! Cooldown...");
                            sleep(Duration::from_secs(60));
                            self.ratelimited_keys = Vec::new();
                            println!("Retrying...");
                        }
                    }
                }

                _ => {
                    let (data, key_status): (Value, KeyStatus) = match self
                        .request(
                            symbol,
                            vec![
                                ("symbol", "SPY"),
                                ("format", "JSON"),
                                ("interval", timeframe),
                                ("outputsize", "5000"),
                                ("end_date", date),
                            ],
                        )
                        .await
                    {
                        Ok(data) => data,
                        Err(err) => return Err(err),
                    };

                    match key_status {
                        KeyStatus::Success => {
                            valid = true;
                            data_parent = data;
                        }
                        KeyStatus::AllKeysFailed => {
                            println!("All keys failed! Cooldown...");
                            sleep(Duration::from_secs(60));
                            self.ratelimited_keys = Vec::new();
                            println!("Retrying...");
                        }
                    }
                }
            }
        }

        Ok(data_parent)
    }

    pub fn export_data_to_csv(&mut self, asset: &str, data: Value) {
        let file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!("data/{}.csv", asset))
            .unwrap();
        let mut writer: Writer<File> = Writer::from_writer(file);

        match asset {
            "SPY" | "VIX" => {
                /*
                writer
                    .write_record(["datetime", "open", "high", "low", "close", "volume"])
                    .expect("Failed to write csv headers");
                */

                if let Value::Array(values) = &data["values"] {
                    for candle in values {
                        if let Value::Object(candle_obj) = candle {
                            let datetime: &str = candle_obj["datetime"].as_str().unwrap_or("");
                            let open: &str = candle_obj["open"].as_str().unwrap_or("");
                            let high: &str = candle_obj["high"].as_str().unwrap_or("");
                            let low: &str = candle_obj["low"].as_str().unwrap_or("");
                            let close: &str = candle_obj["close"].as_str().unwrap_or("");
                            let volume: &str = candle_obj["volume"].as_str().unwrap_or("");

                            writer
                                .write_record([datetime, open, high, low, close, volume])
                                .expect("Failed to write row");
                        }
                    }
                }
            }

            "macd" => {
                /*
                writer
                    .write_record(["datetime", "macd", "macd_signal", "macd_hist"])
                    .expect("Failed to write csv headers");
                */

                if let Value::Array(values) = &data["values"] {
                    for candle in values {
                        if let Value::Object(candle_obj) = candle {
                            let datetime: &str = candle_obj["datetime"].as_str().unwrap_or("");
                            let macd: &str = candle_obj["macd"].as_str().unwrap_or("");
                            let macd_signal: &str =
                                candle_obj["macd_signal"].as_str().unwrap_or("");
                            let macd_hist: &str = candle_obj["macd_hist"].as_str().unwrap_or("");

                            writer
                                .write_record([datetime, macd, macd_signal, macd_hist])
                                .expect("Failed to write row");
                        }
                    }
                }
            }

            "ema_9" | "ema_12" | "ema_26" => {
                /*
                writer
                    .write_record(["datetime", asset])
                    .expect("Failed to write csv headers");
                */
                let mut indicator_name = "";
                if let Some(index) = asset.rfind('_') {
                    indicator_name = &asset[..index];
                }
                if let Value::Array(values) = &data["values"] {
                    for candle in values {
                        if let Value::Object(candle_obj) = candle {
                            let datetime: &str = candle_obj["datetime"].as_str().unwrap_or("");
                            let indicator: &str = candle_obj[indicator_name].as_str().unwrap_or("");

                            writer
                                .write_record([datetime, indicator])
                                .expect("Failed to write row");
                        }
                    }
                }
            }

            _ => {
                /*
                writer
                    .write_record(["datetime", asset])
                    .expect("Failed to write csv headers");
                */
                if let Value::Array(values) = &data["values"] {
                    for candle in values {
                        if let Value::Object(candle_obj) = candle {
                            let datetime: &str = candle_obj["datetime"].as_str().unwrap_or("");
                            let indicator: &str = candle_obj[asset].as_str().unwrap_or("");

                            writer
                                .write_record([datetime, indicator])
                                .expect("Failed to write row");
                        }
                    }
                }
            }
        }

        writer.flush().expect("Failed to flush writer");
    }

    pub async fn get_data_for_nn(&mut self, loops: i32, timeframe: &str) {
        print!("Gathering data...");
        stdout().flush().expect("Failed to flush stdout");

        for asset in self.assets.clone() {
            let mut counter: i32 = 0;

            let current_day: DateTime<Utc> = Utc::now();
            let mut final_datetime: String = current_day.format("%F %R").to_string();

            while counter < loops {
                let data_response: Result<Value, Error> =
                    self.get_data(asset, &final_datetime, timeframe).await;

                match data_response {
                    Ok(data) => {
                        if let Value::Array(values) = &data["values"] {
                            if let Value::Object(candle) =
                                &values.last().expect("Error when getting last value")
                            {
                                let parsed_datetime: Result<NaiveDateTime, ParseError> =
                                    NaiveDateTime::parse_from_str(
                                        candle["datetime"]
                                            .as_str()
                                            .expect("Error when converting Value to &str"),
                                        "%Y-%m-%d %H:%M:%S",
                                    );

                                match parsed_datetime {
                                    Ok(parsed) => {
                                        let utc_datetime: DateTime<Utc> =
                                            DateTime::<Utc>::from_utc(parsed, Utc);
                                        final_datetime = utc_datetime.format("%F %R").to_string();
                                    }

                                    Err(e) => {
                                        println!("Error when parsing datetime: {}", e);
                                        break;
                                    }
                                }
                            }
                        }
                        self.export_data_to_csv(asset, data);
                        counter += 1;
                    }

                    Err(e) => {
                        println!("Error when getting data: {}", e);
                        break;
                    }
                }
            }
        }
        println!("done");
    }

    pub fn merge_csvs(&self) {
        print!("Merging data...");
        stdout().flush().expect("Failed to flush stdout");

        let data_path: &Path = Path::new("data");
        let format_string: &str = "%Y-%m-%d %H:%M:%S";

        let mut output_file: File = File::create(Path::new("data/merged_data.csv"))
            .expect("Failed to create merged csv file");

        let mut datetime_data_map: HashMap<NaiveDateTime, Vec<String>> = HashMap::new();

        for entry in read_dir(data_path).expect("Failed to read data folder") {
            let entry: DirEntry = entry.expect("Failed to get folder entry");
            let entry_path: PathBuf = entry.path();

            if entry.file_name() == "merged_data.csv"
                || entry.file_name() == "merged_parsed_data.csv"
            {
                continue;
            }

            if entry_path.is_file() && entry_path.extension().unwrap_or_default() == "csv" {
                let file: File = File::open(&entry_path).expect("Failed to open file");
                let mut reader: Reader<File> =
                    ReaderBuilder::new().has_headers(false).from_reader(file);

                for result in reader.records() {
                    let record: StringRecord = result.expect("Failed to get csv record");

                    if let Some(datetime) = record.get(0) {
                        if let Ok(parsed_datetime) =
                            NaiveDateTime::parse_from_str(datetime, format_string)
                        {
                            let csv_fields: Vec<String> = record
                                .iter()
                                .skip(1)
                                .map(|field: &str| field.to_string())
                                .collect();

                            datetime_data_map
                                .entry(parsed_datetime)
                                .or_insert_with(Vec::new)
                                .extend(csv_fields);
                        }
                    }
                }
            }
        }

        let mut sorted_data: Vec<NaiveDateTime> = datetime_data_map.keys().cloned().collect();
        sorted_data.sort();

        for datetime in sorted_data {
            if let Some(data) = datetime_data_map.get(&datetime) {
                writeln!(
                    output_file,
                    "{},{}",
                    datetime.format(format_string),
                    data.join(",")
                )
                .expect("Failed to write line to merged csv file");
            }
        }
        println!("done");
    }
}
