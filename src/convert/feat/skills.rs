use json::{array, JsonValue};

pub(crate) fn parse(skills: &JsonValue) -> Option<(JsonValue, u8)> {
    if skills["choose"]["from"].is_array() {
        return Some((skills["choose"]["from"].clone(), 1));
    }
    if skills["any"].is_number() {
        let any = array![
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
        ];
        return skills["any"].as_u8().map(|u| (any, u));
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
