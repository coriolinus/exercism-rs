#[macro_use]
extern crate lazy_static;

use std::ascii::AsciiExt;
use std::collections::HashMap;

lazy_static!{
    static ref LETTER_SCORES: HashMap<char, u32> = {
        let mut m = HashMap::new();

        // one point
        for ch in vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'] {
            m.insert(ch, 1);
        }

        // two
        for ch in vec!['D', 'G'] {
            m.insert(ch, 2);
        }

        // etc
        for ch in vec!['B', 'C', 'M', 'P'] {
            m.insert(ch, 3);
        }

        for ch in vec!['F', 'H', 'V', 'W', 'Y'] {
            m.insert(ch, 4);
        }

        m.insert('K', 5);

        for ch in vec!['J', 'X'] {
            m.insert(ch, 8);
        }

        for ch in vec!['Q', 'Z'] {
            m.insert(ch, 10);
        }

        m
    };
}

pub fn score(word: &str) -> u32 {
    word.chars()
        .map(|ch| match LETTER_SCORES.get(&ch.to_ascii_uppercase()) {
            Some(num) => *num,
            None => 0,
        })
        .fold(0, std::ops::Add::add)
}
