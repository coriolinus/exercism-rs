#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref NON_WORD: Regex = Regex::new(r"[^\w-:;]+").unwrap();
    static ref NUMBER: Regex = Regex::new(r"-?\d+").unwrap();
}

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<i16>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::new() }
    }

    pub fn format_stack(&self) -> String {
        self.stack.iter().enumerate().fold(String::with_capacity(3 * self.stack.len()),
                                           |string, (index, num)| {
            string + &num.to_string() +
            {
                if index != self.stack.len() - 1 {
                    " "
                } else {
                    ""
                }
            }
        })
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        // can't tokenize with a simple input.split_whitespace() because
        // the chars \u{0} and \u{1} from the test case aren't in the
        // Unicode `White_Space` class
        for token in NON_WORD.split(input) {
            if NUMBER.is_match(token) {
                match i16::from_str(token) {
                    Ok(value) => self.stack.push(value),
                    Err(_) => return Err(Error::InvalidWord),
                }
            }
        }
        Ok(())
    }
}
