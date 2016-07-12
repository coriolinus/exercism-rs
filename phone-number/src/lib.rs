pub fn number(phone: &str) -> Option<String> {
    let digits: Vec<char> = phone.chars().filter(|c| c.is_numeric()).collect();
    match digits.len() {
        10 => Some(digits.into_iter().collect()),
        11 if digits[0] == '1' => Some(digits[1..].iter().cloned().collect()),
        _ => None
    }
}

fn phone_sections(phone: &str) -> Option<(String, String, String)> {
    if let Some(ph) = number(phone) {
        let (area_code, rest) = ph.split_at(3);
        let (prefix, suffix) = rest.split_at(3);
        return Some((area_code.to_string(), prefix.to_string(), suffix.to_string()));
    }
    None
}

pub fn area_code(phone: &str) -> Option<String> {
    phone_sections(phone).map(|s| s.0)
}

pub fn pretty_print(phone: &str) -> String {
    match phone_sections(phone) {
        Some((area, pref, suff)) => format!("({}) {}-{}", area, pref, suff),
        None => "invalid".to_string(),
    }
}
