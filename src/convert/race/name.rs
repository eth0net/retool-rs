use json::JsonValue;

const SUFFIXES: [&str; 2] = ["A", "B"];

pub(crate) fn parse(race: &JsonValue, idx: usize, suffix: bool) -> String {
    if suffix {
        format!("{} {} ({})", race["name"], SUFFIXES[idx], race["source"])
    } else {
        format!("{} ({})", race["name"], race["source"])
    }
}
