//! This implementation operates in two separate layers: a low-level layer and a high-level layer.
//!
//! The low-level layer handles the fundamental stack operations: given a stream of primitives,
//! it manipulates the stack appropriately.
//!
//! The high-level layer handles custom definitions. Given a stream of words which may or may not
//! include definitions, it generates a stream of primitives to feed into the low level layer.
//!
//! Architecturally, we're implementing the high-level layer as a generator. This gives us lots
//! of power to recurse and insert an arbitrary number of yielded items, while keeping the
//! interface requirements for the low-level layer minimal.

use high::Definitions;

mod high;
mod low;

pub type Value = i32;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Default)]
pub struct Forth {
    stack: Vec<Value>,
    definitions: Definitions,
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        for primitive in high::evaluate(input, &mut self.definitions) {
            let primitive = primitive?;
            low::evaluate(&mut self.stack, primitive)?;
        }
        Ok(())
    }
}
