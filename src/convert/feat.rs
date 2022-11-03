use anyhow::{bail, Result};
use json::{array, object, JsonValue};

use super::JsonConverter;

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
                desc.push_str(&prerequisite::to_string(feat["prerequisite"].clone()));

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

mod prerequisite {
    use crate::convert::util::{join_conjunct, ordinal_form, title_case};

    use json::JsonValue;

    fn weight(s: &str) -> u8 {
        match s {
            "level" => 0,
            "pact" => 1,
            "patron" => 2,
            "spell" => 3,
            "race" => 4,
            "ability" => 5,
            "proficiency" => 6,
            "spellcasting" => 7,
            "feature" => 8,
            "item" => 9,
            "other" => 10,
            "otherSummary" => 11,
            _ => 12,
        }
    }

    pub(crate) fn to_string(prerequisites: JsonValue) -> String {
        let mut prerequisites: Vec<String> = prerequisites
            .members()
            .filter_map(|prerequisite| {
                // alignment
                // background
                // feat
                // other
                //      armor
                //      weapon
                // psionics

                // todo: sort prerequisite entries by weight
                prerequisite
                    .entries()
                    .filter_map(|(k, v)| match k {
                        "note" => None,
                        "level" => level_string(v),
                        "race" => race_string(v),
                        "ability" => ability_string(v),
                        "proficiency" => proficiency_string(v),
                        "spellcasting" => spellcasting_string(v),
                        "spellcasting2020" => spellcasting2020_string(v),
                        "other" => other_to_string(v),
                        _ => None,
                    })
                    .reduce(|a, i| match i.contains(" or ") {
                        true => format!("{}; {}", a, i),
                        false => format!("{}, {}", a, i),
                    })
            })
            .collect();
        prerequisites.sort_by_key(|a| weight(a));

        let prefix = match prerequisites.len() {
            0 => "",
            1 => "Prerequisite: ",
            _ => "Prerequisites: ",
        };

        format!("{}{}", prefix, prerequisites.join(", "))
    }

    fn other_to_string(v: &JsonValue) -> Option<String> {
        v.as_str().map(|other| other.to_string())
    }

    fn spellcasting2020_string(v: &JsonValue) -> Option<String> {
        match v.as_bool() {
            Some(true) => Some("Spellcasting or Pact Magic feature".to_string()),
            _ => None,
        }
    }

    fn spellcasting_string(v: &JsonValue) -> Option<String> {
        match v.as_bool() {
            Some(true) => Some("The ability to cast at least one spell".to_string()),
            _ => None,
        }
    }

    fn proficiency_string(v: &JsonValue) -> Option<String> {
        let proficiencies = v
            .members()
            .filter_map(|proficiency| {
                let entries = proficiency
                    .entries()
                    .filter_map(|(class, kind)| match class {
                        "armor" => Some(format!("{} {}", kind, class)),
                        "weapon" => Some(format!("a {} {}", kind, class)),
                        _ => None,
                    })
                    .collect::<Vec<String>>();
                join_conjunct(entries, ", ", "and ")
            })
            .collect::<Vec<String>>();
        join_conjunct(proficiencies, ", ", "or ").map(|p| format!("Proficiency with {}", p))
    }

    fn ability_string(v: &JsonValue) -> Option<String> {
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
                acc.0.push(ability.to_string());
                if let Some(lvl) = level.as_usize() {
                    acc.1 = lvl;
                }
                acc
            });
        join_conjunct(abilities, ", ", "or ").map(|a| format!("{} {} or higher", a, level))
    }

    fn race_string(v: &JsonValue) -> Option<String> {
        let races = v
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
        join_conjunct(races, ", ", "or ")
    }

    fn level_string(v: &JsonValue) -> Option<String> {
        if let Some(level) = v.as_u8() {
            return Some(format!("{} level", ordinal_form(level)));
        };
        if let JsonValue::Object(level) = v {
            let level_string = level["level"]
                .as_u8()
                .map(|l| format!("{} level", ordinal_form(l)));

            let class_name = level["class"]["name"].as_str();
            let subclass_name = level["subclass"]["name"].as_str();

            let level_visible = level["level"] != 1;
            let subclass_visible = level["subclass"]["visible"] == true;
            let class_visible = level["class"]["visible"] == true || subclass_visible;

            let class_string: Option<String> = None;

            if level_string.is_some() && class_string.is_some() {
                return Some(format!(
                    "{} {}",
                    level_string.unwrap(),
                    class_string.unwrap()
                ));
            } else if level_string.is_some() {
                return level_string;
            } else if class_string.is_some() {
                return class_string;
            }
        };
        None
    }
}
