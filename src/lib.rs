use std::result;

use json::JsonValue;

mod convert;
mod error;

pub use convert::Converter;
pub use error::Error;

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

/// Alias for std::result::Result with crate::error::Error;
pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{self, File},
        io::Write,
    };

    use json::object;
    use nanoid::nanoid;

    use super::*;

    fn temp_file() -> String {
        let mut path = env::temp_dir();
        path.push(nanoid!());
        path.to_str().expect("path to str ok").to_string()
    }

    #[test]
    fn dummy_convert_file() {
        let input_path = temp_file();
        let output_path = temp_file();

        let input_data = object! {data: "dummy"}.pretty(4);
        File::create(&input_path)
            .expect("input file create ok")
            .write_all(input_data.as_bytes())
            .expect("input file write ok");

        convert_file(Converter::Dummy, &input_path, &output_path).expect("convert_file ok");

        let output_data = fs::read_to_string(output_path).expect("output file read ok");
        assert_eq!(input_data, output_data);
    }

    #[test]
    fn dummy_convert_string() {
        let input = object! { data: "dummy" }.pretty(4);
        let output = convert_string(Converter::Dummy, &input).expect("convert_string ok");
        assert_eq!(input, output);
    }

    #[test]
    fn dummy_convert_json() {
        let input = object! { data: "dummy" };
        let output = convert_json(Converter::Dummy, input.clone()).expect("convert_json ok");
        assert_eq!(input, output);
    }
}
