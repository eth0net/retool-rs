use std::{fs, io::Write, path::PathBuf};

use json::JsonValue;

use crate::Result;

/// Converters to process 5e.tools data into Reroll data.
pub enum Converter {
    /// Dummy converter for testing, simply returns the input data unmodified.
    Dummy,
}

/// Implement Converter methods to handle data processing.
impl Converter {
    /// Convert a JSON file containing an array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_file(&self, input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
        let input_data = fs::read_to_string(input_path)?;

        let output_data = self.convert_string(&input_data)?;

        fs::File::create(output_path)?.write_all(output_data.as_bytes())?;

        Ok(())
    }

    /// Convert a serialised JSON array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_string(&self, input: &str) -> Result<String> {
        let input_json = json::parse(input)?;

        let output_json = self.convert_json(input_json)?;

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
    use std::{env, fs};

    use json::object;
    use nanoid::nanoid;

    use super::*;

    fn temp_file() -> PathBuf {
        let mut path = env::temp_dir();
        path.push(nanoid!());
        path
    }

    #[test]
    fn dummy_convert_file() {
        let input_path = temp_file();
        let output_path = temp_file();

        let input_data = object! { data: "dummy" }.pretty(4);
        fs::File::create(&input_path)
            .expect("failed to create input file")
            .write_all(input_data.as_bytes())
            .expect("failed to write to input file");

        Converter::Dummy
            .convert_file(&input_path, &output_path)
            .expect("failed to convert file");

        let output_data = fs::read_to_string(&output_path).expect("failed to read output file");
        assert_eq!(input_data, output_data);
    }

    #[test]
    fn dummy_convert_string() {
        let input = object! { data: "dummy" }.pretty(4);
        let output = Converter::Dummy
            .convert_string(&input)
            .expect("failed to convert string");
        assert_eq!(input, output);
    }

    #[test]
    fn dummy_convert_json() {
        let input = object! { data: "dummy" };
        let output = Converter::Dummy
            .convert_json(input.clone())
            .expect("failed to convert json");
        assert_eq!(input, output);
    }
}
