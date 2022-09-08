use std::{
    fs::{read_to_string, File},
    io::Write,
};

fn main() {
    let input_path = "./data/input/backgrounds.json";
    let output_path = "./data/output/backgrounds.json";

    let input_data = match read_to_string(input_path) {
        Ok(data) => data,
        Err(error) => panic!("Failed to read input data: {}", error),
    };

    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => panic!("Failed to create output file: {}", error),
    };

    // todo: parse and convert data here

    let output_data = input_data;

    if let Err(error) = output_file.write_all(output_data.as_bytes()) {
        panic!("Failed to write output file: {}", error)
    };
}
