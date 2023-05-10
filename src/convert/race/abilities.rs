use json::array;

use json::JsonValue;

pub(crate) fn parse(abilities: &JsonValue) -> (JsonValue, Vec<JsonValue>) {
    let mut ability_bonuses = array![0, 0, 0, 0, 0, 0];
    let mut ability_choices = vec![];

    abilities
        .entries()
        .for_each(|(ability_key, ability_value)| {
            let index = match ability_key {
                "str" => 0,
                "dex" => 1,
                "con" => 2,
                "int" => 3,
                "wis" => 4,
                "cha" => 5,
                "choose" => {
                    let count = ability_value["count"].as_i8().unwrap_or(0);
                    let amount = ability_value["amount"].as_i8().unwrap_or(1);
                    let mut ability_choice = array![];
                    for _ in 0..count {
                        ability_choice.push(amount).unwrap();
                    }
                    ability_choices.push(ability_choice);
                    return;
                }
                _ => {
                    println!("unknown ability: {}", ability_key);
                    return;
                }
            };

            ability_bonuses[index] = ability_value.clone();
        });

    if ability_choices.is_empty() {
        ability_choices.push(JsonValue::Null);
    }

    (ability_bonuses, ability_choices)
}
