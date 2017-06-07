use std::{fmt, iter};

struct RLEncodeContext {
    ch: Option<char>,
    count: usize,
}

impl RLEncodeContext {
    fn new() -> RLEncodeContext {
        RLEncodeContext {
            ch: None,
            count: 0,
        }
    }

    fn update(&mut self, ch: char) -> Option<String> {
        let rv = match self.ch {
            None => {
                self.ch = Some(ch);
                self.count = 1;
                None
            }
            Some(c) => {
                if c == ch {
                    self.count += 1;
                    None
                } else {
                    Some(self.to_string())
                }
            }
        };
        if rv.is_some() {
            self.ch = Some(ch);
            self.count = 1;
        }
        rv
    }

    fn finish(self) -> String {
        self.to_string()
    }
}


impl fmt::Display for RLEncodeContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ch {
            None => write!(f, ""),
            Some(c) => {
                if self.count == 1 {
                    write!(f, "{}", c)
                } else {
                    write!(f, "{}{}", self.count, c)
                }
            }
        }
    }
}

pub fn encode(input: &str) -> String {
    let mut context = RLEncodeContext::new();
    let mut rv = String::new();
    for ch in input.chars() {
        match context.update(ch) {
            None => {}
            Some(s) => rv += &s,
        }
    }
    rv += &context.finish();
    rv
}

struct RLDecodeContext {
    digits: String,
}

impl RLDecodeContext {
    fn new() -> RLDecodeContext {
        RLDecodeContext { digits: String::new() }
    }

    fn update(&mut self, ch: char) -> Option<String> {
        if ch.is_digit(10) {
            self.digits.push(ch);
            None
        } else {
            if self.digits.is_empty() {
                Some(ch.to_string())
            } else {
                let count =
                    self.digits.parse::<usize>().expect("Non-digit in RLDecodeContext digits");
                self.digits = String::new();
                Some(iter::repeat(ch).take(count).collect::<String>())
            }
        }
    }
}

pub fn decode(input: &str) -> String {
    let mut context = RLDecodeContext::new();
    let mut rv = String::new();
    for ch in input.chars() {
        match context.update(ch) {
            None => {}
            Some(s) => rv += &s,
        }
    }
    rv
}
