use json::{array, JsonValue};

pub(crate) fn parse(skills: &JsonValue) -> Option<(JsonValue, u8)> {
    if skills["choose"]["from"].is_array() {
        return Some((skills["choose"]["from"].clone(), 1));
    }
    if skills["any"].is_number() {
        return skills["any"].as_u8().map(|u| (all_skills(), u));
    }
    let skills: Vec<&str> = skills
        .entries()
        .filter_map(|(k, v)| (v == true).then_some(k))
        .collect();
    match skills.is_empty() {
        false => Some((skills.into(), 0)),
        true => None,
    }
}

fn all_skills() -> JsonValue {
    array![
        "Acrobatics",
        "Animal Handling",
        "Arcana",
        "Athletics",
        "Deception",
        "History",
        "Insight",
        "Intimidation",
        "Investigation",
        "Medicine",
        "Nature",
        "Perception",
        "Performance",
        "Persuasion",
        "Religion",
        "Sleight of Hand",
        "Stealth",
        "Survival",
    ]
}

#[cfg(test)]
mod tests {
    use json::object;

    use super::*;

    #[test]
    fn parse_choose() {
        let input = object! { choose: { from: [ "a", "b" ] }};
        let output = Some((array!["a", "b"], 1));
        assert_eq!(parse(&input), output);
    }

    #[test]
    fn parse_any() {
        let input = object! { any: 1 };
        let output = Some((all_skills(), 1));
        assert_eq!(parse(&input), output);
    }

    #[test]
    fn parse_list() {
        let input = object! { a: true, b: true };
        let output = Some((array!["a", "b"], 0));
        assert_eq!(parse(&input), output);
    }
}
