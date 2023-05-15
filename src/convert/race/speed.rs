use json::{number::Number, JsonValue};

pub(crate) fn for_race(race: &JsonValue) -> Option<Number> {
    match race["speed"] {
        JsonValue::Number(speed) => Some(speed),
        JsonValue::Object(ref speed) => speed["walk"].as_number(),
        _ => None,
    }
}

pub(crate) fn for_subrace(race: &JsonValue, subrace: &JsonValue) -> JsonValue {
    match for_race(subrace) {
        Some(speed) => speed.into(),
        None => race["speed"].clone(),
    }
}

#[cfg(test)]
mod tests {
    use json::object;
    use test_case::test_case;

    use super::*;

    #[test_case(object! { speed: 10 }, Some(10.into()) ; "number")]
    #[test_case(object! { speed: {walk: 20} }, Some(20.into()) ; "object")]
    #[test_case(object! {}, None ; "none")]
    fn test_for_race(input: JsonValue, expected: Option<Number>) {
        assert_eq!(for_race(&input), expected);
    }
}
