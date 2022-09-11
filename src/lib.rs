use std::{
    fmt, fs,
    io::{self, Write},
    result,
};

use json::JsonValue;

/// Convert a JSON file containing an array of 5e.tools entries to the Reroll equivalent.
pub fn convert_file(conv: Converter, input_path: &str, output_path: &str) -> Result<()> {
    conv.convert_file(input_path, output_path)
}

/// Convert a serialised JSON array of 5e.tools entries to the Reroll equivalent.
pub fn convert_string(conv: Converter, input: &str) -> Result<String> {
    conv.convert_string(input)
}

/// Convert a JsonValue array of 5e.tools entries to the Reroll equivalent.
pub fn convert_json(conv: Converter, input: JsonValue) -> Result<JsonValue> {
    conv.convert_json(input)
}

/// Converters to process 5e.tools data into Reroll data.
pub enum Converter {
    Dummy,
}

impl Converter {
    pub fn convert_file(&self, input_path: &str, output_path: &str) -> Result<()> {
        let input_data = fs::read_to_string(input_path)?;

        let output_data = self.convert_string(&input_data)?;

        fs::File::create(output_path)?.write_all(output_data.as_bytes())?;

        Ok(())
    }

    pub fn convert_string(&self, input: &str) -> Result<String> {
        let input_json = json::parse(input)?;

        let output_json = self.convert_json(input_json)?;

        Ok(output_json.pretty(4))
    }

    pub fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        match self {
            Converter::Dummy => DummyConverter.convert_json(input),
        }
    }
}

/// Internal implementation of Converter::Dummy.
struct DummyConverter;

impl DummyConverter {
    fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        Ok(input)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    JsonError(json::Error),
    UnknownKind(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(error) => write!(f, "io error: {}", error),
            Error::JsonError(error) => write!(f, "json error: {}", error),
            Error::UnknownKind(kind) => write!(f, "unknown kind: {}", kind),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<json::Error> for Error {
    fn from(error: json::Error) -> Self {
        Error::JsonError(error)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use json::object;

    use super::*;

    #[test]
    fn dummy_convert_file() {
        let mut input_path_buf = PathBuf::from(file!());
        input_path_buf.pop();
        input_path_buf.push("testdata/dummy.json");
        let input_path = input_path_buf.to_str().expect("input_path_buf.to_str ok");

        let mut output_path_buf = env::temp_dir();
        output_path_buf.push("dummy.json");
        let output_path = output_path_buf.to_str().expect("output_path_buf.to_str ok");

        Converter::Dummy
            .convert_file(input_path, output_path)
            .expect("Converter::Dummy.convert_file ok");

        let input_data = fs::read_to_string(input_path).expect("read input file");
        let output_data = fs::read_to_string(output_path).expect("read output file");

        assert_eq!(input_data, output_data);

        convert_file(Converter::Dummy, input_path, output_path).expect("convert_file ok");

        let input_data = fs::read_to_string(input_path).expect("read input file");
        let output_data = fs::read_to_string(output_path).expect("read output file");
        assert_eq!(input_data, output_data);
    }

    #[test]
    fn dummy_convert_string() {
        let input = object! { key: "value" }.pretty(4);

        let output = Converter::Dummy
            .convert_string(&input)
            .expect("convert_string ok");

        assert_eq!(input, output);
    }

    #[test]
    fn dummy_convert_json() {
        let input = object! { key: "value" };

        let output = Converter::Dummy
            .convert_json(input.clone())
            .expect("convert_json ok");

        assert_eq!(input, output);
    }
}
