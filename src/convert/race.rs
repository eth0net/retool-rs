use anyhow::{bail, Result};
use json::{object, JsonValue};

use super::JsonConverter;

mod abilities;

pub struct RaceConverter;

impl JsonConverter for RaceConverter {
    fn convert_json(&self, input: &JsonValue) -> Result<JsonValue> {
        if !input.has_key("race") {
            bail!("expected race array in object: {{ \"race\": [] }}")
        };

        let output = input["race"]
            .members()
            .flat_map(|race| {
                let (ability_bonuses, ability_choices) = abilities::parse(&race["ability"]);

                // todo: implement traits

                ability_choices
                    .iter()
                    .map(|ability_choice| {
                        object! {
                            name: format!("{} ({})", race["name"], race["source"]),
                            speed: race["speed"]["walk"].as_number().unwrap(),
                            ability_bonuses: ability_bonuses.clone(),
                            flex_ability_bonuses: ability_choice.clone(),
                        }
                    })
                    .collect::<Vec<JsonValue>>()
            })
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
        ability: { str: 1, dex: 2 },
    }, object! {
        name: "Name 1 (Source 1)",
        speed: 10,
        ability_bonuses: [1, 2, 0, 0, 0, 0],
        flex_ability_bonuses: null,
    } ; "str and dex only")]
    #[test_case(object! {
        name: "Name 2",
        source: "Source 2",
        speed: { walk: 20 },
        ability: { con: 3, cha: 6 },
    },
    object! {
        name: "Name 2 (Source 2)".to_string(),
        speed: 20,
        ability_bonuses: [0, 0, 3, 0, 0, 6],
        flex_ability_bonuses: null,
    } ; "con and cha only")]
    #[test_case(object! {
        name: "Name 3",
        source: "Source 3",
        speed: { walk: 30 },
        ability: { int: 4, wis: 5 },
    },
    object! {
        name: "Name 3 (Source 3)".to_string(),
        speed: 30,
        ability_bonuses: [0, 0, 0, 4, 5, 0],
        flex_ability_bonuses: null,
    } ; "int and wis only")]
    #[test_case(object! {
        name: "Name 4",
        source: "Source 4",
        speed: { walk: 40 },
        ability: { choose: { count: 1 }},
    },
    object! {
        name: "Name 4 (Source 4)".to_string(),
        speed: 40,
        ability_bonuses: [0, 0, 0, 0, 0, 0],
        flex_ability_bonuses: [1],
    } ; "choose 1")]
    #[test_case(object! {
        name: "Name 5",
        source: "Source 5",
        speed: { walk: 50 },
        ability: { choose: { count: 2 }},
    },
    object! {
        name: "Name 5 (Source 5)".to_string(),
        speed: 50,
        ability_bonuses: [0, 0, 0, 0, 0, 0],
        flex_ability_bonuses: [1, 1],
    } ; "choose 2")]
    #[test_case(object! {
        name: "Name 6",
        source: "Source 6",
        speed: { walk: 60 },
        ability: { choose: { count: 1, amount: 2 }},
    },
    object! {
        name: "Name 6 (Source 6)".to_string(),
        speed: 60,
        ability_bonuses: [0, 0, 0, 0, 0, 0],
        flex_ability_bonuses: [2],
    } ; "choose 1 amount 2")]
    fn convert_json_test(input: JsonValue, expected: JsonValue) {
        let output = RaceConverter
            .convert_json(&object! { race: [input] })
            .expect("convert json");
        assert_eq!(output, array![expected])
    }
}
