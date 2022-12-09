use anyhow::{bail, Result};
use json::{array, object, JsonValue};

use super::JsonConverter;

mod entries;
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
                let mut desc_stack = vec![];

                if let Some(pr) = prerequisite::to_string(feat["prerequisite"].clone()) {
                    desc_stack.push(pr)
                }

                if let Some(entries) = entries::to_string(feat["entries"].clone()) {
                    desc_stack.push(entries)
                }

                object! {
                    name: feat["name"].to_string(),
                    desc: desc_stack.join("\n\n"),
                    skills_count_choose: 0,
                    skills: array![],
                }
            })
            .collect();

        Ok(JsonValue::Array(output))
    }
}
