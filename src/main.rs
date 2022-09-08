use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    let input_path = "./data/input/backgrounds.json";
    let output_path = "./data/output/backgrounds.json";

    let mut input_file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Failed to open input file: {}", error),
    };

    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => panic!("Failed to create output file: {}", error),
    };

    let mut input_data = String::new();

    if let Err(error) = input_file.read_to_string(&mut input_data) {
        panic!("Failed to read input file: {}", error)
    }

    // todo: parse and convert data here

    let output_data = input_data;

    if let Err(error) = output_file.write_all(output_data.as_bytes()) {
        panic!("Failed to write output file: {}", error)
    };
}
