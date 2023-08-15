use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{prelude::*, stdout, BufReader, Write},
    path::Path,
};

pub fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut transposed_matrix = Vec::with_capacity(num_cols);
    for _ in 0..num_cols {
        transposed_matrix.push(Vec::with_capacity(num_rows))
    }

    for i in 0..num_rows {
        for j in 0..num_cols {
            transposed_matrix[j].push(matrix[i][j].clone());
        }
    }

    transposed_matrix
}

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

    //normalization begins
    let mut transposed: Vec<Vec<String>> = transpose(&output_data).into_iter().filter(|column| !column.iter().all(|value| value == &String::from("0"))).collect();
    for column in transposed.iter_mut().skip(1) {
        let mut row_nums: Vec<f32> = column.iter()
        .map(|s| s.parse::<f32>())
        .filter_map(Result::ok)
        .collect();

        let (max, min): (f32, f32) = row_nums.iter().fold((row_nums[0], row_nums[0]), |(max, min): (f32, f32), &x: &f32| {
            (x.max(max), x.min(min))
        });

        let mut normalized_row: Vec<f32> = Vec::new();

        for num in row_nums.iter_mut() {
            normalized_row.push((*num - min) / (max - min));
        }
    }

    let output_path: &Path = Path::new("data/merged_filtered_data.csv");
    let mut output_file: File = File::create(output_path)?;

    for row in transpose(&transposed) {
        writeln!(output_file, "{}", row.join(","))?;
    }

    println!("done");
    Ok(())
}

