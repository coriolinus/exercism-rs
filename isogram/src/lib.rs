extern crate counter;
use counter::Counter;

use std::ascii::AsciiExt;

pub fn check(input: &str) -> bool {
    let counts = Counter::init(input.chars().map(|c| c.to_ascii_lowercase()));
    counts
        .map
        .iter()
        .filter(|&(&k, _)| k >= 'a' && k <= 'z')
        .all(|(_, &v)| v <= 1)
}
