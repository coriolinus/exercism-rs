use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let letters: HashSet<char> = s.to_lowercase().chars().collect();
    letters.is_superset(&alphabet)
}
