use anyhow::{bail, Result};
use json::{array, object, JsonValue};

use super::entries;
use super::JsonConverter;

mod prerequisite;
mod skills;

pub struct FeatConverter;

impl JsonConverter for FeatConverter {
    fn convert_json(&self, input: &JsonValue) -> Result<JsonValue> {
        if !input.has_key("feat") {
            bail!("expected feat array in object: {{ \"feat\": [] }}")
        };

        let output = input["feat"]
            .members()
            .map(|feat| {
                let mut desc_stack = vec![];

                let mut skill_list = array![];
                let mut skill_count = 0;

                if let Some(pr) = prerequisite::to_string(&feat["prerequisite"]) {
                    desc_stack.push(pr)
                }

                if let Some(entries) = entries::to_string(&feat["entries"]) {
                    desc_stack.push(entries)
                }

                if let Some((l, c)) = skills::parse(&feat["skillProficiencies"][0]) {
                    skill_list = l;
                    skill_count = c;
                }

                object! {
                    name: feat["name"].to_string(),
                    desc: desc_stack.join("\n\n"),
                    skills_count_choose: skill_count,
                    skills: skill_list,
                }
            })
            .collect();

        Ok(JsonValue::Array(output))
    }
}
