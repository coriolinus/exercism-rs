use std::fmt::Display;

extern crate luhn;
use luhn::is_valid;

pub struct Luhn {
    s: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        is_valid(&self.s)
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn { s: input.to_string() }
    }
}
