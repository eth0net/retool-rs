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
                let mut desc_stack = vec![];

                if let Some(pr) = prerequisite::to_string(feat["prerequisite"].clone()) {
                    desc_stack.push(pr)
                }

                if let Some(entries) = join_entries(feat["entries"].clone()) {
                    desc_stack.push(entries)
                }

                object! {
                    name: feat["name"].clone(),
                    desc: desc_stack.join("\n"),
                    skills_count_choose: 0,
                    skills: array![],
                }
            })
            .collect();

        Ok(JsonValue::Array(output))
    }
}

fn join_entries(entries: JsonValue) -> Option<String> {
    let entries: Vec<String> = entries
        .members()
        .filter_map(|entry| match entry {
            JsonValue::String(entry) => Some(entry.clone()),
            _ => None,
        })
        .collect();
    match entries.is_empty() {
        false => Some(entries.join("\n")),
        true => None,
    }
}
