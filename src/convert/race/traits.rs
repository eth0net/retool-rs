use json::{object, JsonValue};

use crate::convert::entries;

pub(crate) fn for_race(traits: &JsonValue) -> JsonValue {
    let to_trait = |t: &JsonValue| {
        object! {
            name: t["name"].to_string(),
            desc: entries::to_string(&t["entries"]),
        }
    };

    traits
        .members()
        .map(to_trait)
        .collect::<Vec<JsonValue>>()
        .into()
}

pub(crate) fn for_subrace(race: &JsonValue, subrace: &JsonValue) -> JsonValue {
    let race_traits = &race["traits"];
    let subrace_traits = for_race(&subrace["entries"]);

    race_traits
        .members()
        .chain(subrace_traits.members())
        .cloned()
        .collect::<Vec<JsonValue>>()
        .into()
}

#[cfg(test)]
mod tests {
    use json::array;
    use test_case::test_case;

    use super::*;

    #[test_case(object! {
        name: "Trait",
        entries: ["Entry 1", "Entry 2"],
    }, object! {
        name: "Trait",
        desc: "Entry 1\n\nEntry 2",
    } ; "string entries")]
    #[test_case(object! {
        name: "Trait",
        entries: [
            object! {
                name: "Name 1",
                type: "entries",
                entries: ["Text 1"],
            },
            object! {
                name: "Name 2",
                type: "entries",
                entries: ["Text 2"],
            },
        ],
    }, object! {
        name: "Trait",
        desc: "Name 1. Text 1\n\nName 2. Text 2",
    } ; "object entries")]
    #[test_case(object! {
        name: "Trait",
        entries: [
            object! {
                name: "Name 1",
                type: "list",
                items: [
                    object! {
                        type: "item",
                        entry: "Text 1",
                    },
                ],
            },
            object! {
                name: "Name 2",
                type: "list",
                items: [
                    object! {
                        type: "item",
                        entry: "Text 2",
                    },
                ],
            },
        ],
    }, object! {
        name: "Trait".to_string(),
        desc: "Name 1\n- Text 1\n\nName 2\n- Text 2",
    } ; "list entries")]
    #[test_case(object! {
        name: "Trait",
        entries: [
            object! {
                caption: "Caption 1",
                type: "table",
                colLabels: ["Col 1", "Col 2"],
                rows: [
                    ["Row 1 Col 1", "Row 1 Col 2"],
                    ["Row 2 Col 1", "Row 2 Col 2"],
                ],
            },
        ],
    }, object! {
        name: "Trait".to_string(),
        desc: "Caption 1\n+-------------+-------------+\n| Col 1       | Col 2       |\n+===========================+\n| Row 1 Col 1 | Row 1 Col 2 |\n|-------------+-------------|\n| Row 2 Col 1 | Row 2 Col 2 |\n+-------------+-------------+",
    } ; "mixed entries")]
    fn test_for_race(input: JsonValue, expected: JsonValue) {
        assert_eq!(for_race(&array![input]), array![expected]);
    }
}
