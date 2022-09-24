use std::{fs, io::Write, path::Path};

use anyhow::{bail, Context, Result};
use json::{array, object, JsonValue};

/// Converters to process 5e.tools data into Reroll data.
pub enum Converter {
    /// Converter sample for testing that returns the input data unmodified.
    Dummy,
    /// Converter to process feats from 5e.tools into Reroll data.
    Feat,
}

/// Implement Converter methods to handle data processing.
impl Converter {
    /// Convert a JSON file containing an array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        let input_data = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read input file: {}", input_path.display()))?;

        let output_data = self
            .convert_string(&input_data)
            .with_context(|| "Failed to convert string")?;

        fs::File::create(output_path)
            .with_context(|| format!("Failed to create output file: {}", output_path.display()))?
            .write_all(output_data.as_bytes())
            .with_context(|| "Failed to write to output file")?;

        Ok(())
    }

    /// Convert a serialised JSON array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_string(&self, input: &str) -> Result<String> {
        let input_json = json::parse(input).with_context(|| "Failed to parse JSON")?;

        let output_json = self
            .convert_json(input_json)
            .with_context(|| "Failed to convert json")?;

        Ok(output_json.pretty(4))
    }

    /// Convert a JsonValue array of 5e.tools entries to the Reroll equivalent.
    pub fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        match self {
            Converter::Dummy => DummyConverter.convert_json(input),
            Converter::Feat => FeatConverter.convert_json(input),
        }
    }
}

trait JsonConverter {
    fn convert_json(&self, input: JsonValue) -> Result<JsonValue>;
}

struct DummyConverter;

impl JsonConverter for DummyConverter {
    fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        Ok(input)
    }
}

struct FeatConverter;

impl JsonConverter for FeatConverter {
    fn convert_json(&self, input: JsonValue) -> Result<JsonValue> {
        if !input.has_key("feat") {
            bail!("expected feat array in object: {{ \"feat\": [] }}")
        };

        let output = input["feat"]
            .members()
            .map(|feat| {
                let mut desc = String::new();
                desc.push_str(&self.prerequisite_to_string(feat["prerequisite"].clone()));

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

impl FeatConverter {
    fn prerequisite_to_string(&self, from: JsonValue) -> String {
        // Prerequisite: Elf or half-elf
        // Prerequisite: Half-Elf, half-orc, or human
        // Prerequisite: Dwarf or a Small race
        // Prerequisite: Charisma 13 or higher
        // Prerequisite: Intelligence or Wisdom 13 or higher
        // Prerequisites: 4th level, strixhaven initiate feat
        // Prerequisite: Proficiency with light armor
        // Prerequisite: The ability to cast at least one spell
        // Prerequisite: Spellcasting or Pact Magic feature

        from.members().fold(String::from("Prerequisite: "), |a, i| {
            let mut r = String::new();

            if !i["ability"].is_empty() {
                let mut abilities = vec![];
                let mut level = 0;

                ABILITIES.iter().for_each(|ability| {
                    if i["ability"].has_key(ability.key) {
                        abilities.push(ability.value);
                        if let Some(lvl) = i["ability"][ability.key].as_i32() {
                            level = lvl
                        }
                    }
                });

                r.push_str(&format!("{} {} or higher", abilities.join(" or "), level))
            }

            if !i["race"].is_empty() {
                let to_name = |race: &JsonValue| {
                    let mut name = race["name"].to_string();
                    if race.has_key("displayEntry") {
                        name = race["displayEntry"].to_string();
                    }
                    if race.has_key("subrace") {
                        name.push_str(&format!(" ({})", race["subrace"]))
                    }
                    name
                };

                let races: Vec<String> = i["race"].members().map(to_name).collect();

                r.push_str(&races.join(", "))
            }

            // alignment
            // background
            // feat
            // level
            //      class
            // other
            // proficiency
            //      armor
            //      weapon
            // psionics
            // spellcasting
            // spellcasting2020

            a + &r
        })
    }
}

struct KeyValuePair<'a> {
    key: &'a str,
    value: &'a str,
}

const ABILITIES: [KeyValuePair; 6] = [
    KeyValuePair {
        key: "cha",
        value: "Charisma",
    },
    KeyValuePair {
        key: "con",
        value: "Constitution",
    },
    KeyValuePair {
        key: "dex",
        value: "Dexterity",
    },
    KeyValuePair {
        key: "int",
        value: "Intelligence",
    },
    KeyValuePair {
        key: "str",
        value: "Strength",
    },
    KeyValuePair {
        key: "wis",
        value: "Wisdom",
    },
];

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

        let input_data = object! { data: "dummy" }.pretty(4);
        input_file
            .write_str(input_data.as_str())
            .expect("write input data");

        Converter::Dummy
            .convert_file(input_file.path(), output_file.path())
            .expect("convert file");

        let output_data = fs::read_to_string(output_file.path()).expect("read output file");
        assert_eq!(input_data, output_data);
    }

    #[test]
    fn dummy_convert_string() {
        let input = object! { data: "dummy" }.pretty(4);
        let output = Converter::Dummy
            .convert_string(&input)
            .expect("convert string");
        assert_eq!(input, output);
    }

    #[test]
    fn dummy_convert_json() {
        let input = object! { data: "dummy" };
        let output = Converter::Dummy
            .convert_json(input.clone())
            .expect("convert json");
        assert_eq!(input, output);
    }
}
