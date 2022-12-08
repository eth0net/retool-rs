use json::JsonValue;

pub(crate) fn to_string(entries: JsonValue) -> Option<String> {
    let entries: Vec<String> = entries.members().filter_map(entry_to_string).collect();

    match entries.is_empty() {
        false => Some(entries.join("\n")),
        true => None,
    }
}

fn entry_to_string(entry: &JsonValue) -> Option<String> {
    match entry {
        JsonValue::String(e) => Some(e.to_string()),
        JsonValue::Object(e) => match &e["type"] {
            t if t == "entries" => None,
            t if t == "list" => list_to_string(&e["items"]),
            t if t == "section" => None,
            t if t == "table" => None,
            _ => None,
        },
        _ => None,
    }
}

fn list_to_string(items: &JsonValue) -> Option<String> {
    let items: Vec<String> = items
        .members()
        .filter_map(entry_to_string)
        .map(|i| format!("- {}", i))
        .collect();

    match items.is_empty() {
        false => Some(items.join("\n")),
        true => None,
    }
}
