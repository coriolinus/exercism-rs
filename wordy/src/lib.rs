use std::ops;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operation {
    fn operate_on<T>(&self, left: T, right: T) -> T
        where T: ops::Add + ops::Sub + ops::Mul + ops::Div +
            From<<T as ops::Add>::Output> +
            From<<T as ops::Sub>::Output> +
            From<<T as ops::Mul>::Output> +
            From<<T as ops::Div>::Output>
    {
        use Operation::*;
        match *self {
            Add => T::from(left + right),
            Sub => T::from(left - right),
            Mul => T::from(left * right),
            Div => T::from(left / right),
            Pow => unimplemented!(),
        }
    }
}

pub struct WordProblem {
    command: String,
}

impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        WordProblem {command: command.to_string() }
    }

    pub fn answer(&self) -> Result<isize, &'static str> {
        unimplemented!();
    }

}
