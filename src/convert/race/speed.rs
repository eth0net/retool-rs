use json::{number::Number, JsonValue};

pub(crate) fn parse(race: &JsonValue) -> Option<Number> {
    match race["speed"] {
        JsonValue::Number(speed) => Some(speed),
        JsonValue::Object(ref speed) => speed["walk"].as_number(),
        _ => None,
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
    fn test_parse(input: JsonValue, expected: Option<Number>) {
        assert_eq!(parse(&input), expected);
    }
}
