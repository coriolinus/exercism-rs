// for iterator .join() method
extern crate itertools;
use itertools::Itertools;

pub fn abbreviate(phrase: &str) -> String {
    phrase.trim()
        .split(|c: char| !c.is_alphabetic())
        .filter(|&word| word.len() > 0)
        .map(|word| {
            if word.chars().all(|ch| ch.is_uppercase()) {
                word.chars().next().unwrap().to_string()
            } else {
                let mut initial = true;
                word.chars()
                    .filter(|&ch| {
                        if initial {
                            initial = false;
                            true
                        } else if ch.is_uppercase() {
                            true
                        } else {
                            false
                        }
                    })
                    .collect::<String>()
            }
        })
        .join("")
        .to_uppercase()
}
