use json::array;

use json::object;
use json::JsonValue;

pub(crate) fn apply(race: &JsonValue) -> JsonValue {
    let mut race = race.clone();
    if race["lineage"] == "VRGR" || race["lineage"] == "UA1" {
        race["entries"].push(object! {
            type: "entries",
            name: "Languages",
            entries: ["You can speak, read, and write Common and one other language that you and your DM agree is appropriate for your character."],
        }).unwrap();
    }

    match &race["lineage"] {
        t if t == "VRGR" => {
            race["ability"] = array![
                { choose: { weighted: { weights: [2,1] }}},
                { choose: { weighted: { weights: [1,1,1] }}},
            ];
        }
        t if t == "UA1" => {
            race["ability"] = array![
                { choose: { weighted: { weights: [2,1] }}},
            ];
        }
        _ => {}
    };

    race
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(
        object! {
            name: "Name 1",
            source: "Source 1",
            speed: { walk: 10 },
            ability: [{ str: 1, dex: 2 }],
            entries: [],
        },
        object! {
            name: "Name 1",
            source: "Source 1",
            speed: { walk: 10 },
            ability: [{ str: 1, dex: 2 }],
            entries: [],
        }
        ; "no lineage"
    )]
    #[test_case(
        object! {
            name: "Name 1",
            source: "Source 1",
            speed: { walk: 10 },
            ability: [{ str: 1, dex: 2 }],
            entries: [],
            lineage: "VRGR",
        },
        object! {
            name: "Name 1",
            source: "Source 1",
            speed: { walk: 10 },
            ability: [{ choose: { weighted: { weights: [2,1] }}}, { choose: { weighted: { weights: [1,1,1] }}} ],
            entries: [{
                type: "entries",
                name: "Languages",
                entries: ["You can speak, read, and write Common and one other language that you and your DM agree is appropriate for your character."],
            }],
            lineage: "VRGR",
        }
        ; "VRGR"
    )]
    #[test_case(
        object! {
            name: "Name 1",
            source: "Source 1",
            speed: { walk: 10 },
            ability: [{ str: 1, dex: 2 }],
            entries: [],
            lineage: "UA1",
        },
        object! {
            name: "Name 1",
            source: "Source 1",
            speed: { walk: 10 },
            ability: [{ choose: { weighted: { weights: [2,1] }}} ],
            entries: [{
                type: "entries",
                name: "Languages",
                entries: ["You can speak, read, and write Common and one other language that you and your DM agree is appropriate for your character."],
            }],
            lineage: "UA1",
        }
        ; "UA1"
    )]
    fn test_apply(input: JsonValue, expected: JsonValue) {
        assert_eq!(apply(&input), expected)
    }
}
