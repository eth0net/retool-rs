use comfy_table::Table;
use json::JsonValue;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) fn to_string(entries: &JsonValue) -> Option<String> {
    let entries: Vec<String> = entries.members().filter_map(entry_to_string).collect();

    match entries.is_empty() {
        false => Some(entries.join("\n\n")),
        true => None,
    }
}

fn entry_to_string(entry: &JsonValue) -> Option<String> {
    match entry {
        JsonValue::Short(e) => Some(format_string(e.to_string())),
        JsonValue::String(e) => Some(format_string(e.to_string())),
        JsonValue::Object(e) => match &e["type"] {
            t if t == "entries" => item_to_string(entry),
            t if t == "item" => item_to_string(entry),
            t if t == "list" => list_to_string(entry),
            t if t == "section" => item_to_string(entry),
            t if t == "table" => table_to_string(entry),
            t if t == "itemSpell" => item_to_string(entry),
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

fn table_to_string(table: &JsonValue) -> Option<String> {
    let mut stack = vec![];
    let mut tbl = Table::new();

    tbl.set_header(table["colLabels"].members());
    table["rows"].members().for_each(|r| {
        tbl.add_row(r.members().map(|m| m.to_string()).map(format_string));
    });

    if let Some(caption) = entry_to_string(&table["caption"]) {
        stack.push(caption);
    }

    let tbl = tbl.to_string();
    if !tbl.is_empty() {
        stack.push(tbl);
    }

    match stack.is_empty() {
        false => Some(stack.join("\n")),
        true => None,
    }
}

fn format_string(s: String) -> String {
    lazy_static! {
        static ref RE_CHANCE: Regex = Regex::new(r"\{@chance .*?(?P<chance>(?:\w| )+?)\}").unwrap();
        static ref RE_TAGS: Regex =
            Regex::new(r"\{@(?:(?:\w| )+?) (?P<value>[a-zA-Z0-9-_',./+ ]+?)(?:\|.*?)?\}").unwrap();
        static ref RE_NOTE: Regex = Regex::new(r"\{@note (?P<note>.*?)?\}").unwrap();
    }

    let s = RE_CHANCE.replace_all(&s, "$chance");
    let s = RE_TAGS.replace_all(&s, "$value");
    RE_NOTE.replace_all(&s, "$note").to_string()
}
