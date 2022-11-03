pub(crate) fn ordinal_form(i: u8) -> String {
    let m = (i % 10, i % 100);
    let suffix = if m.0 == 1 && m.1 != 11 {
        "st"
    } else if m.0 == 2 && m.1 != 12 {
        "nd"
    } else if m.0 == 3 && m.1 != 13 {
        "rd"
    } else {
        "th"
    };
    format!("{}{}", i, suffix)
}

pub(crate) fn title_case(s: &str) -> String {
    let to_title = |s: &str| {
        let (first, rest) = s.split_at(1);
        format!(
            "{}{}",
            first.to_ascii_uppercase(),
            rest.to_ascii_lowercase()
        )
    };

    s.split('-')
        .map(to_title)
        .collect::<Vec<String>>()
        .join("-")
}

pub(crate) fn join_conjunct(v: Vec<String>, s1: &str, s2: &str) -> Option<String> {
    v.iter()
        .enumerate()
        .map(|(i, s)| match i {
            _ if i == 0 => s.to_string(),
            _ if i == v.len() - 1 => format!("{}{}{}", s1, s2, s),
            _ => format!("{}{}", s1, s),
        })
        .reduce(|a, i| format!("{}{}", a, i))
}
