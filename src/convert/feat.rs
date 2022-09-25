use anyhow::{bail, Result};
use json::{array, object, JsonValue};

use super::*;

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
            let mut r = vec![];

            if !i["ability"].is_empty() {
                let (abilities, level) =
                    i["ability"].members().fold((vec![], 0), |mut acc, member| {
                        member.entries().fold(&mut acc, |acc, entry| {
                            // todo: break out this match logic into Option<&str> fn
                            // then to if let Some(...) and wrap the level mod too
                            match entry.0 {
                                "cha" => acc.0.push("Charisma"),
                                "con" => acc.0.push("Constitution"),
                                "dex" => acc.0.push("Dexterity"),
                                "int" => acc.0.push("Intelligence"),
                                "str" => acc.0.push("Strength"),
                                "wis" => acc.0.push("Wisdom"),
                                _ => return acc,
                            };
                            if let Some(lvl) = entry.1.as_usize() {
                                acc.1 = lvl
                            }
                            acc
                        });
                        acc
                    });

                r.push(format!("{} {} or higher", abilities.join(" or "), level))
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

                let mut races: Vec<String> = i["race"].members().map(to_name).collect();
                let len = races.len();
                if len > 2 {
                    races[len - 1].insert_str(0, "or ");
                }

                r.push(races.join(", "))
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

            a + &r.join(", ")
        })
    }
}
