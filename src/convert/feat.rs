use anyhow::{bail, Result};
use json::{array, object, JsonValue};

use super::JsonConverter;

mod prerequisite;

pub struct FeatConverter;

impl JsonConverter for FeatConverter {
    fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        if !input.has_key("feat") {
            bail!("expected feat array in object: {{ \"feat\": [] }}")
        };

        let output = input["feat"]
            .members()
            .map(|feat| {
                let mut desc = String::new();

                if let Some(pr) = prerequisite::to_string(feat["prerequisite"].clone()) {
                    desc.push_str(pr.as_str())
                }

                object! {
                    name: feat["name"].clone(),
                    desc: desc,
                    skills_count_choose: 0,
                    skills: array![],
                }
            })
            .collect();

        Ok(JsonValue::Array(output))
    }
}
