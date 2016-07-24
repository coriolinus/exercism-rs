use std::str::FromStr;

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
        for token in input.split_whitespace() {
            if token.chars().all(|c| c.is_numeric()) {
                match i16::from_str(token) {
                    Ok(value) => self.stack.push(value),
                    Err(_) => return Err(Error::InvalidWord),
                }
            }
        }
        Ok(())
    }
}
