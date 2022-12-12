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
