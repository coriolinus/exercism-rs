
use std::ops;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn operate_on<T>(&self, left: T, right: T) -> T
        where T:
            ops::Add<Output=T> +
            ops::Sub<Output=T> +
            ops::Mul<Output=T> +
            ops::Div<Output=T>
    {
        use Operation::*;
        match *self {
            Add => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
        }
    }
}

pub struct WordProblem {
    command: String,
}

impl WordProblem {
    // Not a fan of this style; I'd greatly prefer the constructor
    // pub fn new(command: &str) -> Option<WordProblem>
    //
    // I get that it makes testing a little simpler to do it like this,
    // but in real life, I want errors to show up as soon as possible,
    // so that you can fix them immediately. This implies parsing
    // problems like this in the constructor, so that the errors at least
    // stay local. Otherwise, you have invalid WordProblems floating around
    // within your code, just waiting for someone to call `.answer()`
    // somewhere entirely nonlocal, before they strike.
    pub fn new(command: &str) -> WordProblem {
        WordProblem {command: command.to_string() }
    }

    pub fn answer(&self) -> Result<isize, &'static str> {
        match WordProblem::parse(&self.command) {
            Some((left, operation, right)) => {
                Ok(operation.operate_on(left, right))
            }
            None => Err("Can't parse that command!"),
        }
    }

    fn parse(command: &str) -> Option<(isize, Operation, isize)> {
        unimplemented!()
    }

}
