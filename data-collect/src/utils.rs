use crate::config::{BASE_URL, KEYS, NUM_KEYS};
use chrono::{prelude::*, ParseError};
use csv::{Reader, ReaderBuilder, StringRecord, StringRecordIter, StringRecordsIter, Writer};
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

pub fn filter_csv() {
    let data_path: &Path = Path::new("data/merged_data.csv");

    if data_path.is_file() && data_path.extension().unwrap_or_default() == "csv" {
        let file: File = File::open(data_path).expect("Failed to open file");
        let mut reader: Reader<File> = ReaderBuilder::new().has_headers(false).from_reader(file);
        let data: Vec<Vec<String>> = reader
            .records()
            .map(|record| {
                record
                    .expect("Failed to get result")
                    .iter()
                    .map(|field| field.to_string())
                    .collect()
            })
            .collect::<Vec<Vec<String>>>();

        let longest_row_length: usize = data
            .iter()
            .max_by_key(|row| row.len())
            .expect("Failed to get longest row")
            .len();
    }
}
