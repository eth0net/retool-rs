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
        JsonValue::Short(e) => Some(e.to_string()),
        JsonValue::String(e) => Some(e.to_string()),
        JsonValue::Object(e) => match &e["type"] {
            t if t == "entries" => None,
            t if t == "item" => item_to_string(entry),
            t if t == "list" => list_to_string(entry),
            t if t == "section" => None,
            t if t == "table" => None,
            _ => None,
        },
        JsonValue::Array(_) => {
            let entries: Vec<String> = entry.members().filter_map(entry_to_string).collect();
            match entries.is_empty() {
                false => Some(entries.join("\n")),
                true => None,
            }
        }
        _ => None,
    }
}

fn item_to_string(item: &JsonValue) -> Option<String> {
    let mut stack = vec![];

    if let Some(name) = entry_to_string(&item["name"]) {
        stack.push(format!("{}.", name));
    }

    if let Some(entry) = entry_to_string(&item["entry"]) {
        stack.push(entry);
    }

    if let Some(entries) = entry_to_string(&item["entries"]) {
        stack.push(entries);
    }

    match stack.is_empty() {
        false => Some(stack.join(" ")),
        true => None,
    }
}

fn list_to_string(list: &JsonValue) -> Option<String> {
    let mut stack = vec![];

    if let Some(name) = entry_to_string(&list["name"]) {
        stack.push(name);
    }

    let pfx = match &list["style"].to_string().contains("list-hang") {
        true => "",
        false => "- ",
    };

    list["items"]
        .members()
        .filter_map(entry_to_string)
        .map(|i| format!("{}{}", pfx, i))
        .for_each(|i| stack.push(i));

    match stack.is_empty() {
        false => Some(stack.join("\n")),
        true => None,
    }
}
