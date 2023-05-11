use json::{number::Number, JsonValue};

pub(crate) fn to_string(race: &JsonValue) -> Option<Number> {
    match race["speed"] {
        JsonValue::Number(speed) => Some(speed),
        JsonValue::Object(ref speed) => speed["walk"].as_number(),
        _ => None,
    }
}
