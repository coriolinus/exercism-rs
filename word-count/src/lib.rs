use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut ret = HashMap::new();
    for word in s.split(" ") {
        let word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
        if word.len() > 0 {
            let counter = ret.entry(word).or_insert(0);
            *counter += 1;
        }
    }

    ret
}
