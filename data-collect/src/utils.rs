use csv::{Reader, ReaderBuilder};
use std::{
    fs::File,
    path::Path
};

pub fn filter_csv() {
    let data_path: &Path = Path::new("data/merged_data.csv");

    if data_path.is_file() && data_path.extension().unwrap_or_default() == "csv" {
        let file: File = File::open(data_path).expect("Failed to open file");
        let mut reader: Reader<File> = ReaderBuilder::new().has_headers(false).from_reader(file);
        let mut data: Vec<Vec<String>> = reader
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


        for (index, row) in &mut data.iter().enumerate() {
            if row.len() < longest_row_length {
                data.remove(index);
            }
        }
    }
}
