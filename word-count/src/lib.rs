use std::collections::HashMap;

fn allowed_char(c: &char) -> bool {
    c.is_alphabetic() || c.is_numeric()
}

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut ret = HashMap::new();
    for word in s.to_lowercase().split(|c: char| !(allowed_char(&c))) {
        let word: String = word.chars().filter(allowed_char).collect();
        if word.len() > 0 {
            let counter = ret.entry(word).or_insert(0);
            *counter += 1;
        }
    }

    ret
}
