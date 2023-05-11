use json::JsonValue;

const SUFFIXES: [&str; 2] = ["A", "B"];

pub(crate) fn parse(race: &JsonValue, idx: usize, suffix: bool) -> Option<String> {
    if suffix && idx >= SUFFIXES.len() {
        return None;
    }

    let name = if suffix {
        format!("{} {} ({})", race["name"], SUFFIXES[idx], race["source"])
    } else {
        format!("{} ({})", race["name"], race["source"])
    };

    Some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    use json::object;
    use test_case::test_case;

    #[test_case("Name", "Source", 0, false, "Name (Source)"; "no suffix")]
    #[test_case("Name", "Source", 0, true, "Name A (Source)"; "suffix a")]
    #[test_case("Name", "Source", 1, true, "Name B (Source)"; "suffix b")]
    #[test_case("Name", "Source", 2, false, "Name (Source)"; "out of bounds no suffix")]
    fn test_parse(name: &str, source: &str, idx: usize, suffix: bool, expected: &str) {
        let race = object! { name: name, source: source };
        assert_eq!(parse(&race, idx, suffix), Some(expected.to_string()))
    }

    #[test]
    fn test_parse_index_out_of_bounds() {
        assert_eq!(parse(&object! {}, 2, true), None)
    }
}
