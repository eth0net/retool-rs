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
    use crate::convert::util::{join_conjunct, title_case};

    use json::JsonValue;
    use ordinal::Ordinal;

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
                // background
                // feat
                // other
                //      armor
                //      weapon
                // psionics

                prerequisite
                    .entries()
                    .filter_map(|(k, v)| match k {
                        "ability" => ability_string(v),
                        "alignment" => alignment_string(v),
                        "level" => level_string(v),
                        "note" => None,
                        "other" => other_to_string(v),
                        "proficiency" => proficiency_string(v),
                        "race" => race_string(v),
                        "spellcasting" => spellcasting_string(v),
                        "spellcasting2020" => spellcasting2020_string(v),
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

    fn alignment_string(v: &JsonValue) -> Option<String> {
        let abv_str = |alignment| match alignment {
            "L" => Some("lawful"),
            "C" => Some("chaotic"),
            "G" => Some("good"),
            "E" => Some("evil"),
            "N" => Some("neutral"),
            "NX" => Some("neutral (law/chaos axis)"),
            "NY" => Some("neutral (good/evil axis)"),
            "U" => Some("unaligned"),
            "A" => Some("any alignment"),
            _ => None,
        };

        let alignment = v
            .members()
            .filter_map(|j| j.as_str())
            .collect::<Vec<&str>>();

        match alignment.len() {
            1 => abv_str(alignment[0]).map(|a| a.to_string()),
            2 => {
                let al = alignment
                    .iter()
                    .filter_map(|a| abv_str(*a))
                    .collect::<Vec<&str>>();
                match al.is_empty() {
                    false => Some(al.join(", ")),
                    true => None,
                }
            }
            3 if alignment.contains(&"NX")
                && alignment.contains(&"NY")
                && alignment.contains(&"N") =>
            {
                Some("any neutral alignment".to_string())
            }
            4 if !alignment.contains(&"L") && !alignment.contains(&"NX") => {
                Some("any chaotic alignment".to_string())
            }
            4 if !alignment.contains(&"C") && !alignment.contains(&"NX") => {
                Some("any lawful alignment".to_string())
            }
            4 if !alignment.contains(&"G") && !alignment.contains(&"NY") => {
                Some("any evil alignment".to_string())
            }
            4 if !alignment.contains(&"E") && !alignment.contains(&"NY") => {
                Some("any good alignment".to_string())
            }
            5 if !alignment.contains(&"L") => Some("any non-lawful alignment".to_string()),
            5 if !alignment.contains(&"C") => Some("any non-chaotic alignment".to_string()),
            5 if !alignment.contains(&"G") => Some("any non-good alignment".to_string()),
            5 if !alignment.contains(&"E") => Some("any non-evil alignment".to_string()),
            _ => None,
        }
    }

    fn level_string(v: &JsonValue) -> Option<String> {
        if let Some(level) = v.as_u8() {
            return Some(format!("{} level", Ordinal(level)));
        };

        if let JsonValue::Object(level) = v {
            let mut stack = vec![];

            if let Some(lvl) = level["level"].as_u8() {
                if lvl != 1 {
                    stack.push(format!("{} level", Ordinal(lvl)))
                }
            }

            if let Some(class) = level["class"]["name"].as_str() {
                if level["class"]["visible"] == true {
                    stack.push(class.to_string());
                }
            }

            if let Some(subclass) = level["subclass"]["name"].as_str() {
                if level["subclass"]["visible"] == true {
                    stack.push(subclass.to_string())
                }
            }

            return Some(stack.join(" "));
        };

        None
    }

    fn other_to_string(v: &JsonValue) -> Option<String> {
        v.as_str().map(|other| other.to_string())
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

    fn spellcasting_string(v: &JsonValue) -> Option<String> {
        match v.as_bool() {
            Some(true) => Some("The ability to cast at least one spell".to_string()),
            _ => None,
        }
    }

    fn spellcasting2020_string(v: &JsonValue) -> Option<String> {
        match v.as_bool() {
            Some(true) => Some("Spellcasting or Pact Magic feature".to_string()),
            _ => None,
        }
    }
}
