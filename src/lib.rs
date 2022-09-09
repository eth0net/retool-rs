use std::{fs, io::Write};

pub fn convert_file(input_path: &str, output_path: &str) -> Result<()> {
    let input_data = fs::read_to_string(input_path)?;

    // todo: parse and convert data here

    let output_data = input_data;

    fs::File::create(output_path)?.write_all(output_data.as_bytes())?;

    Ok(())
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
