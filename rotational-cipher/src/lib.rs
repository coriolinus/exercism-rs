pub fn rotate_char(c: char, i: u8) -> char {
    if c.is_alphabetic() {
        let basis = if c.is_uppercase() { 'A' } else { 'a' };
        ((((c as u8) - (basis as u8) + i) % 26) + (basis as u8)) as char
    } else {
        c
    }
}

pub fn rotate(s: &str, i: u8) -> String {
    s.chars().map(|c| rotate_char(c, i)).collect()
}
