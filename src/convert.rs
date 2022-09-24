use std::{fs, io::Write, path::Path};

use anyhow::{Context, Result};
use json::JsonValue;

/// Converters to process 5e.tools data into Reroll data.
pub enum Converter {
    /// Dummy converter for testing, simply returns the input data unmodified.
    Dummy,
}

/// Implement Converter methods to handle data processing.
impl Converter {
    /// Convert a JSON file containing an array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        let input_data = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read input file: {}", input_path.display()))?;

        let output_data = self
            .convert_string(&input_data)
            .with_context(|| "Failed to convert string")?;

        fs::File::create(output_path)
            .with_context(|| format!("Failed to create output file: {}", output_path.display()))?
            .write_all(output_data.as_bytes())
            .with_context(|| "Failed to write to output file")?;

        Ok(())
    }

    /// Convert a serialised JSON array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_string(&self, input: &str) -> Result<String> {
        let input_json = json::parse(input).with_context(|| "Failed to parse JSON")?;

        let output_json = self
            .convert_json(input_json)
            .with_context(|| "Failed to convert json")?;

        Ok(output_json.pretty(4))
    }

    /// Convert a JsonValue array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        match self {
            Converter::Dummy => DummyConverter.convert_json(input),
        }
    }
}

struct DummyConverter;

impl DummyConverter {
    pub(crate) fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use assert_fs::{prelude::*, NamedTempFile};
    use json::object;

    use super::*;

    #[test]
    fn dummy_convert_file() {
        let input_file = NamedTempFile::new("input.json").expect("create input file");
        let output_file = NamedTempFile::new("output.json").expect("create output file");

        let input_data = object! { data: "dummy" }.pretty(4);
        input_file
            .write_str(input_data.as_str())
            .expect("write input data");

        Converter::Dummy
            .convert_file(input_file.path(), output_file.path())
            .expect("convert file");

        let output_data = fs::read_to_string(output_file.path()).expect("read output file");
        assert_eq!(input_data, output_data);
    }

    #[test]
    fn dummy_convert_string() {
        let input = object! { data: "dummy" }.pretty(4);
        let output = Converter::Dummy
            .convert_string(&input)
            .expect("convert string");
        assert_eq!(input, output);
    }

    #[test]
    fn dummy_convert_json() {
        let input = object! { data: "dummy" };
        let output = Converter::Dummy
            .convert_json(input.clone())
            .expect("convert json");
        assert_eq!(input, output);
    }
}
