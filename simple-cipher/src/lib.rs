extern crate rand;
use rand::distributions::{Distribution, Uniform};

const A: i8 = 'a' as i8;

fn shift<O>(key: &str, s: &str, op: O) -> Option<String>
where
    O: Fn((i8, i8)) -> i8,
{
    if key.len() == 0 {
        return None;
    }
    let mut out = String::with_capacity(s.len());
    for (s_c, key_c) in s.chars().zip(key.chars().cycle()) {
        if !s_c.is_alphabetic() || !s_c.is_ascii_lowercase() || !key_c.is_alphabetic()
            || !key_c.is_ascii_lowercase()
        {
            return None;
        }
        let s_d = s_c as i8 - A;
        let key_d = key_c as i8 - A;
        let r_d = (26 + op((s_d, key_d))) % 26;
        out.push((r_d + A) as u8 as char)
    }
    Some(out)
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    shift(key, s, |(k, s)| k + s)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    shift(key, s, |(k, s)| k - s)
}

pub fn encode_random(s: &str) -> (String, String) {
    use std::cmp::max;

    let letters = Uniform::new_inclusive('a' as u8, 'z' as u8);
    let mut rng = rand::thread_rng();

    let key_len = max(s.len(), 100);
    let mut key = String::with_capacity(key_len);
    for _ in 0..key_len {
        key.push(letters.sample(&mut rng) as char);
    }
    let encoded = encode(&key, s).unwrap_or(String::new());
    (key, encoded)
}
