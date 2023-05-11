use anyhow::{bail, Result};
use json::{object, JsonValue};

use super::JsonConverter;

mod abilities;
mod name;
mod speed;
mod traits;

pub struct RaceConverter;

impl JsonConverter for RaceConverter {
    fn convert_json(&self, input: &JsonValue) -> Result<JsonValue> {
        if !input.has_key("race") {
            bail!("expected race array in object: {{ \"race\": [] }}")
        };

        let output = input["race"]
            .members()
            .filter_map(|race| {
                if race["traitTags"].contains("NPC Race") {
                    return None;
                }

                let (ability_bonuses, ability_choices) = abilities::parse(race);
                let add_suffix = ability_choices.len() > 1;

                let res = ability_choices
                    .iter()
                    .enumerate()
                    .map(|(idx, ability_choice)| {
                        object! {
                            name: name::parse(race, idx, add_suffix),
                            speed: speed::parse(race),
                            ability_bonuses: ability_bonuses.clone(),
                            flex_ability_bonuses: ability_choice.clone(),
                            traits: traits::parse(&race["entries"]),
                        }
                    })
                    .collect::<Vec<JsonValue>>();
                Some(res)
            })
            .flatten()
            .collect();

        Ok(JsonValue::Array(output))
    }
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
