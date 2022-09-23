use std::path::Path;

use anyhow::Result;
use json::JsonValue;

mod convert;

pub use convert::Converter;

/// Convert a JSON file containing an array of 5e.tools entries to the Reroll equivalent.
pub fn convert_file(conv: Converter, input_path: &Path, output_path: &Path) -> Result<()> {
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

        let input_data = object! {data: "dummy"}.pretty(4);
        input_file
            .write_str(input_data.as_str())
            .expect("write input data");

        convert_file(Converter::Dummy, input_file.path(), output_file.path())
            .expect("convert file");

        let output_data = fs::read_to_string(output_file.path()).expect("read output file");
        assert_eq!(input_data, output_data);
    }

    #[test]
    fn dummy_convert_string() {
        let input = object! { data: "dummy" }.pretty(4);
        let output = convert_string(Converter::Dummy, &input).expect("convert string");
        assert_eq!(input, output);
    }

    #[test]
    fn dummy_convert_json() {
        let input = object! { data: "dummy" };
        let output = convert_json(Converter::Dummy, input.clone()).expect("convert json");
        assert_eq!(input, output);
    }
}
