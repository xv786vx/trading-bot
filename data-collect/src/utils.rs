use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{prelude::*, stdout, BufReader, Write},
    path::Path,
};

pub fn filter_merged_csv() -> Result<(), Box<dyn Error>> {
    print!("Filtering merged data...");
    stdout().flush()?;

    let input_path: &Path = Path::new("data/merged_data.csv");
    let input_file: File = File::open(input_path)?;
    let input_reader: BufReader<File> = BufReader::new(input_file);
    let input_data: Vec<Vec<String>> = input_reader
        .lines()
        .map(|line| line.unwrap().split(',').map(|s| s.to_string()).collect())
        .collect();

    let length_count: HashMap<usize, usize> =
        input_data
            .iter()
            .map(|row| row.len())
            .fold(HashMap::new(), |mut map, len| {
                *map.entry(len).or_insert(0) += 1;
                map
            });

    let longest_row_mode: usize = length_count
        .iter()
        .max_by_key(|&(_, frequency)| frequency)
        .unwrap()
        .0
        .to_owned();

    let output_data: Vec<Vec<String>> = input_data
        .into_iter()
        .filter(|row| row.len() == longest_row_mode)
        .collect();

    let output_path: &Path = Path::new("data/merged_filtered_data.csv");
    let mut output_file: File = File::create(output_path)?;

    for row in output_data {
        writeln!(output_file, "{}", row.join(","))?;
    }

    println!("done");
    Ok(())
}
