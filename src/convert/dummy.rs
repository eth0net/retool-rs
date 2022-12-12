use anyhow::Result;
use json::JsonValue;

use super::*;

pub struct DummyConverter;

impl JsonConverter for DummyConverter {
    fn convert_json(&self, input: &JsonValue) -> Result<JsonValue> {
        Ok(input.clone())
    }
}
