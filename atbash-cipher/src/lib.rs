use std::ascii::AsciiExt;

pub fn atbash(ch: char) -> char {
    let tch = ch.to_ascii_lowercase();
    if tch >= 'a' && tch <= 'z' {
        ('z' as u8 - (tch as u8 - 'a' as u8)) as char
    } else {
        ch
    }
}

fn is_allowed_char(ch: &char) -> bool {
    ch.is_alphanumeric() && ch.is_ascii()
}

pub fn encode(s: &str) -> String {
    // Not a huge fan of all the repetitive
    // .collect() calls, or the .cloned in the .map,
    // but that's the only way it'll compile, unfortunately.
    s.chars()
        .filter(is_allowed_char)
        .map(atbash)
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(s: &str) -> String {
    s.chars()
        .filter(is_allowed_char)
        .map(atbash)
        .collect()
}
