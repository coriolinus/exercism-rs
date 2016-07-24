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
        self.stack.iter().fold(String::new(), |string, num| string + &num.to_string())
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        unimplemented!()
    }
}
