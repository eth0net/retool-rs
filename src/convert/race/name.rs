use json::JsonValue;

const SUFFIXES: [&str; 2] = ["A", "B"];

pub(crate) fn for_race(race: &JsonValue, idx: usize, suffix: bool) -> Option<String> {
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

pub(crate) fn for_subrace(race: &JsonValue, subrace: &JsonValue) -> String {
    let name = race["name"].to_string();
    let race_name = subrace["raceName"].to_string();
    let subrace_name = subrace["name"].to_string();
    let full_name = format!("{} ({})", race_name, subrace_name);
    name.replace(&race_name, &full_name)
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
    fn test_for_race(name: &str, source: &str, idx: usize, suffix: bool, expected: &str) {
        let race = object! { name: name, source: source };
        assert_eq!(for_race(&race, idx, suffix), Some(expected.to_string()))
    }

    #[test]
    fn test_for_race_index_out_of_bounds() {
        assert_eq!(for_race(&object! {}, 2, true), None)
    }

    #[test_case("Race (Source)", "Race", "sub", "Race (sub) (Source)"; "without variants")]
    #[test_case("Race A (Source)", "Race", "sub", "Race (sub) A (Source)"; "with variants")]
    fn test_for_subrace(name: &str, race: &str, subrace: &str, expected: &str) {
        let input_race = object! { name: name };
        let input_subrace = object! { name: subrace, raceName: race };
        assert_eq!(for_subrace(&input_race, &input_subrace), expected)
    }
}
