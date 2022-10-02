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

            if let Some(other) = i["other"].as_str() {
                r.push(other.to_string());
            }

            if let Some(level) = i["level"].as_usize() {
                let suffix = match level % 10 {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                };
                r.push(format!("{}{} level", level, suffix));
            }

            if let JsonValue::Array(abilities) = &i["ability"] {
                let (abilities, level) = abilities
                    .iter()
                    .flat_map(|ability| ability.entries())
                    .map(|ability| match ability.0 {
                        "cha" => ("Charisma", ability.1),
                        "con" => ("Constitution", ability.1),
                        "dex" => ("Dexterity", ability.1),
                        "int" => ("Intelligence", ability.1),
                        "str" => ("Strength", ability.1),
                        "wis" => ("Wisdom", ability.1),
                        _ => ("Unknown", ability.1),
                    })
                    .fold((vec![], 0), |mut acc, ability| {
                        acc.0.push(ability.0);
                        if let Some(lvl) = ability.1.as_usize() {
                            acc.1 = lvl;
                        }
                        acc
                    });

                r.push(format!("{} {} or higher", abilities.join(" or "), level))
            }

            if let JsonValue::Array(races) = &i["race"] {
                let to_name = |race: &JsonValue| {
                    let name = match race["displayEntry"].is_string() {
                        true => race["displayEntry"].to_string(),
                        false => race["name"].to_string(),
                    };

                    match race["subrace"].is_string() {
                        true => format!("{} ({})", name, race["subrace"]),
                        false => name,
                    }
                };

                let mut races = races.iter().map(to_name).collect::<Vec<String>>();

                if let Some(race) = races.get(0) {
                    let to_title = |s: &str| {
                        let (first, rest) = s.split_at(1);
                        format!(
                            "{}{}",
                            first.to_ascii_uppercase(),
                            rest.to_ascii_lowercase()
                        )
                    };

                    races[0] = race
                        .split('-')
                        .map(to_title)
                        .collect::<Vec<String>>()
                        .join("-");
                }

                let len = races.len();
                let sep = match len > 2 {
                    true => {
                        races[len - 1].insert_str(0, "or ");
                        ", "
                    }
                    false => " or ",
                };

                r.push(races.join(sep));
            }

            if i["proficiency"].is_array() {
                let to_strings = |proficiency: &JsonValue| {
                    proficiency
                        .entries()
                        .map(|(class, kind)| match class {
                            "weapon" => format!("a {} {}", kind, class),
                            _ => format!("{} {}", kind, class),
                        })
                        .collect::<Vec<String>>()
                };

                let join_strings = |mut v: Vec<String>| -> String {
                    let mut sep = " and ";
                    let len = v.len();
                    if len > 2 {
                        v[len - 1].insert_str(0, "and ");
                        sep = ", ";
                    }
                    v.join(sep)
                };

                let mut proficiencies = i["proficiency"]
                    .members()
                    .map(to_strings)
                    .map(join_strings)
                    .collect::<Vec<String>>();

                let mut sep = " or ";
                let len = proficiencies.len();
                if len > 2 {
                    proficiencies[len - 1].insert_str(0, "or ");
                    sep = ", ";
                }

                r.push(format!("Proficiency with {}", proficiencies.join(sep)));
            }

            if let Some(spellcasting) = i["spellcasting"].as_bool() {
                if spellcasting {
                    r.push("The ability to cast at least one spell".to_string())
                }
            }

            if let Some(spellcasting2020) = i["spellcasting2020"].as_bool() {
                if spellcasting2020 {
                    r.push("Spellcasting or Pact Magic feature".to_string())
                }
            }

            // alignment
            // background
            // feat
            // other
            //      armor
            //      weapon
            // psionics

            a + &r.join(", ")
        })
    }
}
