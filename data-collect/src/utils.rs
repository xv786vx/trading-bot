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




pub fn filter_csv() {
    let data_path: &Path = Path::new("data/merged_data.csv");


    //let entry = read_dir(data_path).expect("Failed to get file entry");

    if data_path.is_file() && data_path.extension().unwrap_or_default() == "csv" {
        let file: File = File::open(&data_path).expect("Failed to open file");
        let mut reader: Reader<File> = ReaderBuilder::new().has_headers(false).from_reader(file);

        if let Some(max_columns_row) = reader.records().into_iter().max_by_key(|row| row.unwrap().len()) {
            for result in reader.records() {
                let record: StringRecord = result.expect("Failed to get csv record");
                if record.len() != max_columns_row.unwrap().clone().len() {
            
                }
            }
            
        }
        
    }
}