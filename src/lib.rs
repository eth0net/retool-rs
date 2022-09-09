use std::{fs, io::Write};

use json::JsonValue;

pub fn convert_file(input_path: &str, output_path: &str) -> Result<()> {
    DummyConverter::new().convert_file(input_path, output_path)
}

pub trait Convert {
    fn convert_file(&self, input_path: &str, output_path: &str) -> Result<()> {
        let input_data = fs::read_to_string(input_path)?;

        let output_data = self.convert_string(&input_data)?;

        fs::File::create(output_path)?.write_all(output_data.as_bytes())?;

        Ok(())
    }

    fn convert_string(&self, input: &str) -> Result<String> {
        let input_json = json::parse(input)?;

        let output_json = self.convert_json(input_json)?;

        Ok(output_json.pretty(2))
    }

    fn convert_json(&self, input: JsonValue) -> Result<JsonValue>;
}

struct DummyConverter {}

impl DummyConverter {
    fn new() -> DummyConverter {
        DummyConverter {}
    }
}

impl Convert for DummyConverter {
    fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        Ok(input)
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
