use json::array;

use json::JsonValue;

pub(crate) fn parse(abilities: &JsonValue) -> Vec<(JsonValue, JsonValue)> {
    abilities.members().map(map_ability).collect()
}

fn map_ability(ability: &JsonValue) -> (JsonValue, JsonValue) {
    let mut ability_bonuses = array![0, 0, 0, 0, 0, 0];
    let mut ability_choices = JsonValue::Null;

    ability.entries().for_each(|(key, value)| {
        match key {
            "str" => ability_bonuses[0] = value.clone(),
            "dex" => ability_bonuses[1] = value.clone(),
            "con" => ability_bonuses[2] = value.clone(),
            "int" => ability_bonuses[3] = value.clone(),
            "wis" => ability_bonuses[4] = value.clone(),
            "cha" => ability_bonuses[5] = value.clone(),
            "choose" => ability_choices = parse_choices(value),
            _ => println!("unknown ability: {}", key),
        };
    });

    (ability_bonuses, ability_choices)
}

fn parse_choices(value: &JsonValue) -> JsonValue {
    if value["weighted"].is_object() {
        return value["weighted"]["weights"].clone();
    }

    if let Some(count) = value["count"].as_u8() {
        let amount = value["amount"].as_u8().unwrap_or(1);
        let mut choices = array![];
        for _ in 0..count {
            choices.push(amount).unwrap();
        }
        return choices;
    }

    JsonValue::Null
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(
        array![{ str: 1, dex: 2}] ,
        vec![(array![1,2,0,0,0,0], JsonValue::Null)]
        ; "str and dex only"
    )]
    #[test_case(
        array![{ con: 3, cha: 4}] ,
        vec![(array![0,0,3,0,0,4], JsonValue::Null)]
        ; "con and cha only"
    )]
    #[test_case(
        array![{ int: 5, wis: 6}] ,
        vec![(array![0,0,0,5,6,0], JsonValue::Null)]
        ; "int and wis only"
    )]
    #[test_case(
        array![{ choose: { count: 1 }}],
        vec![(array![0,0,0,0,0,0], array![1])]
        ; "choose 1"
    )]
    #[test_case(
        array![{ choose: { count: 2 }}],
        vec![(array![0,0,0,0,0,0], array![1,1])]
     ; "choose 2"
    )]
    #[test_case(
        array![{ choose: { count: 1, amount: 2 }}],
        vec![(array![0,0,0,0,0,0], array![2])]
        ; "choose 1 amount 2"
    )]
    #[test_case(
        array![
            { choose: { weighted: { weights: [2,1] }}},
            { choose: { weighted: { weights: [1,1,1] }}},
        ],
        vec![
            (array![0,0,0,0,0,0], array![2,1]),
            (array![0,0,0,0,0,0], array![1,1,1]),
        ]
        ; "lineage VRGR"
    )]
    #[test_case(
        array![
            { choose: { weighted: { weights: [2,1] }}},
        ],
        vec![(array![0,0,0,0,0,0], array![2,1])]
        ; "lineage UA1"
    )]
    fn test_parse(input: JsonValue, expected: Vec<(JsonValue, JsonValue)>) {
        assert_eq!(parse(&input), expected);
    }
}
