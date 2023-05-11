use json::array;

use json::JsonValue;

pub(crate) fn parse(race: &JsonValue) -> (JsonValue, Vec<JsonValue>) {
    let mut ability_bonuses = array![0, 0, 0, 0, 0, 0];
    let mut ability_choices = vec![];

    race["ability"][0]
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

    match &race["lineage"] {
        t if t == "VRGR" => {
            ability_choices.push(array![2, 1]);
            ability_choices.push(array![1, 1, 1]);
        }
        t if t == "UA1" => {
            ability_choices.push(array![2, 1]);
        }
        _ => {}
    }

    if ability_choices.is_empty() {
        ability_choices.push(JsonValue::Null);
    }

    (ability_bonuses, ability_choices)
}

#[cfg(test)]
mod tests {
    use json::object;
    use test_case::test_case;

    use super::*;

    #[test_case(
        object!{ ability: [{ str: 1, dex: 2}] },
        (array![1, 2, 0, 0, 0, 0], vec![JsonValue::Null])
        ; "str and dex only"
    )]
    #[test_case(
        object!{ ability: [{ con: 3, cha: 4}] },
        (array![0, 0, 3, 0, 0, 4], vec![JsonValue::Null])
        ; "con and cha only"
    )]
    #[test_case(
        object!{ ability: [{ int: 5, wis: 6}] },
        (array![0, 0, 0, 5, 6, 0], vec![JsonValue::Null])
        ; "int and wis only"
    )]
    #[test_case(
        object!{ ability: [{ choose: { count: 1 }}]},
        (array![0, 0, 0, 0, 0, 0], vec![array![1]])
        ; "choose 1"
    )]
    #[test_case(
        object!{ ability: [{ choose: { count: 2 }}]},
        (array![0, 0, 0, 0, 0, 0], vec![array![1,1]])
        ; "choose 2"
    )]
    #[test_case(
        object!{ ability: [{ choose: { count: 1, amount: 2 }}]},
        (array![0, 0, 0, 0, 0, 0], vec![array![2]])
        ; "choose 1 amount 2"
    )]
    #[test_case(
        object!{ ability: [{ str: 1, dex: 2 }], lineage: "VRGR" },
        (array![1, 2, 0, 0, 0, 0], vec![array![2,1], array![1,1,1]])
        ; "lineage VRGR"
    )]
    #[test_case(
        object!{ ability: [{ con: 3, cha: 4 }], lineage: "UA1" },
        (array![0, 0, 3, 0, 0, 4], vec![array![2,1]])
        ; "lineage UA1"
    )]
    fn test_parse(input: JsonValue, expected: (JsonValue, Vec<JsonValue>)) {
        assert_eq!(parse(&input), expected);
    }
}
