use anyhow::{bail, Result};
use json::{object, JsonValue};

use super::JsonConverter;

mod abilities;
mod lineage;
mod name;
mod speed;
mod traits;

pub struct RaceConverter;

impl JsonConverter for RaceConverter {
    fn convert_json(&self, input: &JsonValue) -> Result<JsonValue> {
        if !input.has_key("race") {
            bail!("expected race array in object: {{ \"race\": [] }}")
        };

        let races = input["race"]
            .members()
            .filter_map(map_races)
            .flatten()
            .collect();

        Ok(JsonValue::Array(races))
    }
}

fn map_races(race: &JsonValue) -> Option<Vec<JsonValue>> {
    if race["traitTags"].contains("NPC Race") {
        println!("Warning: skipping race {}: npc race", race["name"]);
        return None;
    }

    if !race["_copy"].is_null() {
        println!(
            "Warning: skipping race {}: _copy not supported",
            race["name"]
        );
        return None;
    }

    let race = lineage::apply(race);

    let abilities = abilities::parse(&race["ability"]);
    let add_suffix = abilities.len() > 1;

    let abilities = abilities
        .iter()
        .enumerate()
        .map(|(idx, (bonuses, choices))| {
            object! {
                name: name::parse(&race, idx, add_suffix),
                speed: speed::parse(&race),
                ability_bonuses: bonuses.clone(),
                flex_ability_bonuses: choices.clone(),
                traits: traits::parse(&race["entries"]),
            }
        })
        .collect();

    Some(abilities)
}

#[cfg(test)]
mod tests {
    use super::*;

    use json::array;
    use test_case::test_case;

    #[test_case(object! {
        name: "Name 1",
        source: "Source 1",
        speed: { walk: 10 },
        ability: [{ str: 1, dex: 2 }],
        entries: [],
    }, array![{
        name: "Name 1 (Source 1)",
        speed: 10,
        ability_bonuses: [1, 2, 0, 0, 0, 0],
        flex_ability_bonuses: null,
        traits: [],
    }] ; "simple bonus")]
    #[test_case(object! {
        name: "Name 2",
        source: "Source 2",
        speed: { walk: 20 },
        ability: [{ choose: { count: 1, amount: 2 }}],
        entries: [],
    },
    array![{
        name: "Name 2 (Source 2)".to_string(),
        speed: 20,
        ability_bonuses: [0, 0, 0, 0, 0, 0],
        flex_ability_bonuses: [2],
        traits: [],
    }] ; "choose bonus")]
    #[test_case(object! {
        name: "Name 3",
        source: "Source 3",
        speed: { walk: 30 },
        traitTags: ["NPC Race"],
        ability: [],
        entries: [],
    },
    array![] ; "skip npcs")]
    fn convert_json_test(input: JsonValue, expected: JsonValue) {
        let output = RaceConverter
            .convert_json(&object! { race: [input] })
            .expect("convert json");
        assert_eq!(output, expected)
    }
}
