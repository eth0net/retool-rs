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
    fn prerequisite_to_string(&self, prerequisites: JsonValue) -> String {
        let prerequisites: Vec<String> = prerequisites
            .members()
            // todo: sort prerequisites by weight
            .filter_map(|prerequisite| {
                // alignment
                // background
                // feat
                // other
                //      armor
                //      weapon
                // psionics

                prerequisite
                    .entries()
                    .filter_map(|(k, v)| match k {
                        "note" => None,
                        "level" => {
                            if let Some(lvl) = v.as_u8() {
                                let suffix = ordinal_form(lvl);
                                return Some(format!("{} level", suffix));
                            }
                            if let Some(lvl) = v["level"].as_u8() {
                                let suffix = ordinal_form(lvl);
                                return Some(format!("{} level", suffix));
                            }
                            None
                        }
                        "race" => {
                            let mut races = v
                                .members()
                                .filter_map(|race| {
                                    let display_entry = race["displayEntry"].as_str();
                                    let name = race["name"].as_str();
                                    let subrace = race["subrace"].as_str();

                                    display_entry.or(name).map(|race| (race, subrace))
                                })
                                .map(|(race, subrace)| (title_case(race), subrace))
                                .map(|(race, subrace)| match subrace {
                                    Some(subrace) => format!("{} ({})", race, subrace),
                                    None => race,
                                })
                                .collect::<Vec<String>>();

                            let len = races.len();
                            let sep = match len > 2 {
                                true => {
                                    races[len - 1].insert_str(0, "or ");
                                    ", "
                                }
                                false => " or ",
                            };

                            Some(races.join(sep))
                        }
                        "ability" => {
                            let (abilities, level) = v
                                .members()
                                .flat_map(|ability| ability.entries())
                                .filter_map(|(ability, level)| match ability {
                                    "cha" => Some(("Charisma", level)),
                                    "con" => Some(("Constitution", level)),
                                    "dex" => Some(("Dexterity", level)),
                                    "int" => Some(("Intelligence", level)),
                                    "str" => Some(("Strength", level)),
                                    "wis" => Some(("Wisdom", level)),
                                    _ => None,
                                })
                                .fold((vec![], 0), |mut acc, (ability, level)| {
                                    acc.0.push(ability);
                                    if let Some(lvl) = level.as_usize() {
                                        acc.1 = lvl;
                                    }
                                    acc
                                });

                            match !abilities.is_empty() {
                                true => {
                                    Some(format!("{} {} or higher", abilities.join(" or "), level))
                                }
                                false => None,
                            }
                        }
                        "proficiency" => {
                            let mut proficiencies = v
                                .members()
                                .map(|proficiency| {
                                    proficiency
                                        .entries()
                                        .map(|(class, kind)| match class {
                                            "weapon" => format!("a {} {}", kind, class),
                                            _ => format!("{} {}", kind, class),
                                        })
                                        .collect::<Vec<String>>()
                                })
                                .map(|mut v| {
                                    let mut sep = " and ";
                                    let len = v.len();
                                    if len > 2 {
                                        v[len - 1].insert_str(0, "and ");
                                        sep = ", ";
                                    }
                                    v.join(sep)
                                })
                                .collect::<Vec<String>>();

                            let mut sep = " or ";
                            let len = proficiencies.len();
                            if len > 2 {
                                proficiencies[len - 1].insert_str(0, "or ");
                                sep = ", ";
                            }

                            Some(format!("Proficiency with {}", proficiencies.join(sep)))
                        }
                        "spellcasting" => match v.as_bool() {
                            Some(true) => {
                                Some(String::from("The ability to cast at least one spell"))
                            }
                            _ => None,
                        },
                        "spellcasting2020" => match v.as_bool() {
                            Some(true) => Some(String::from("Spellcasting or Pact Magic feature")),
                            _ => None,
                        },
                        "other" => v.as_str().map(|other| other.to_string()),
                        _ => None,
                    })
                    .reduce(|a, i| match i.contains(" or ") {
                        true => format!("{}; {}", a, i),
                        false => format!("{}, {}", a, i),
                    })
            })
            .collect();

        let prefix = match prerequisites.len() {
            1 => "Prerequisite: ",
            _ => "Prerequisites: ",
        };

        format!("{}{}", prefix, prerequisites.join(", "))
    }
}

fn ordinal_form(i: u8) -> String {
    let m = (i % 10, i % 100);
    let suffix = if m.0 == 1 && m.1 != 11 {
        "st"
    } else if m.0 == 2 && m.1 != 12 {
        "nd"
    } else if m.0 == 3 && m.1 != 13 {
        "rd"
    } else {
        "th"
    };
    format!("{}{}", i, suffix)
}

fn title_case(s: &str) -> String {
    let to_title = |s: &str| {
        let (first, rest) = s.split_at(1);
        format!(
            "{}{}",
            first.to_ascii_uppercase(),
            rest.to_ascii_lowercase()
        )
    };

    s.split('-')
        .map(to_title)
        .collect::<Vec<String>>()
        .join("-")
}
