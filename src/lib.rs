use std::{
    fmt, fs,
    io::{self, Write},
    result,
};

use json::JsonValue;

/// Convert a JSON file containing an array of 5e.tools entries to the Reroll equivalent.
/// Any resulting data will writen to a JSON file created at the output path.
pub fn convert_file(kind: ConverterKind, input_path: &str, output_path: &str) -> Result<()> {
    Converter::new(kind).convert_file(input_path, output_path)
}

/// Convert serialised JSON array of 5e.tools entries to the Reroll equivalent.
pub fn convert_string(kind: ConverterKind, input: &str) -> Result<String> {
    Converter::new(kind).convert_string(input)
}

/// Convert a JsonValue array of 5e.tools entries to the Reroll equivalent.
pub fn convert_json(kind: ConverterKind, input: JsonValue) -> Result<JsonValue> {
    Converter::new(kind).convert_json(input)
}

/// Converters to process 5e.tools data into Reroll data.
pub enum ConverterKind {
    Dummy,
}

pub struct Converter {
    kind: ConverterKind,
}

impl Converter {
    /// Create a new Converter of type 'kind'.
    pub fn new(kind: ConverterKind) -> Converter {
        Converter { kind }
    }

    pub fn convert_file(&self, input_path: &str, output_path: &str) -> Result<()> {
        let input_data = fs::read_to_string(input_path)?;

        let output_data = self.convert_string(&input_data)?;

        fs::File::create(output_path)?.write_all(output_data.as_bytes())?;

        Ok(())
    }

    pub fn convert_string(&self, input: &str) -> Result<String> {
        let input_json = json::parse(input)?;

        let output_json = self.convert_json(input_json)?;

        Ok(output_json.pretty(2))
    }

    pub fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        match self.kind {
            ConverterKind::Dummy => DummyConverter.convert_json(input),
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
