use json::JsonValue;

pub(crate) fn to_string(entries: JsonValue) -> Option<String> {
    let entries: Vec<String> = entries.members().filter_map(map_entries).collect();

    match entries.is_empty() {
        false => Some(entries.join("\n")),
        true => None,
    }
}

fn map_entries(entry: &JsonValue) -> Option<String> {
    let mut stack = vec![];

    match entry {
        JsonValue::String(e) => stack.push(e.to_string()),
        JsonValue::Object(e) => {
            match e["type"].to_string() {
                t if t == "entries" => (),
                t if t == "list" => stack.push(list_to_string(&e["items"])),
                t if t == "section" => (),
                t if t == "table" => (),
                _ => (),
            };
        }
        _ => (),
    }

    match stack.is_empty() {
        false => Some(stack.join("\n")),
        true => None,
    }
}

fn list_to_string(items: &JsonValue) -> String {
    items
        .members()
        .filter_map(|m| m.as_str())
        .collect::<Vec<&str>>()
        .join(",")
}
