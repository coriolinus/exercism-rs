// extern crate num;
use std::ops;

// The following isn't stable as of Rust 1.10, current as of this writing.
//
// The ? operator is not stable.
// TryFrom and TryInto are not stable.
//
// I'm not motivated enough to download nightly just to get this going.
//
// ------------------------
//
// trait Pow<T, U> {
//     fn pow(self, rhs: U) -> T;
// }
//
// impl<T, U> Pow<T, U> for T
//     where T: num::One<Output=T> + ops::Mul<T> + Clone,
//         usize: TryFrom<U>
//         {
//     fn pow(self, rhs: U) -> Result<T, <U as TryInto>::Err> {
//         num::pow::pow(self, rhs.try_into()?)
//     }
// }

#[derive(Copy, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
//    Pow,
}

impl Operation {
    fn operate_on<T>(&self, left: T, right: T) -> T
        where T:
            ops::Add<Output=T> +
            ops::Sub<Output=T> +
            ops::Mul<Output=T> +
            ops::Div<Output=T> +
//            Pow<T, T>
    {
        use Operation::*;
        match *self {
            Add => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
//            Pow => left.pow(right),
        }
    }
}

#[derive(Copy, Clone)]
enum ProblemItem {
    Oper(Operation),
    Val(isize),
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
        let pis = WordProblem::parse(&self.command);
        if pis.is_none() {
            return Err("Can't parse that command!");
        }
        let mut pis = pis.unwrap();
        if pis.len() < 3 || pis.len() % 2 == 0 {
            return Err("Wrong number of problem items");
        }
        if let ProblemItem::Val(mut left) = pis.remove(0) {
            while pis.len() >= 2 {
                if let (ProblemItem::Oper(oper), ProblemItem::Val(right)) = (pis.remove(0), pis.remove(0)) {
                    left = oper.operate_on(left, right);
                } else {
                    return Err("Couldn't parse a problem item as expected type");
                }
            }
            Ok(left)
        } else {
            Err("First problem item wasn't a value")
        }
    }

    fn parse(command: &str) -> Option<Vec<ProblemItem>> {
        let prefix = "What is ";
        let suffix = "?";

        if !command.starts_with(prefix) || !command.ends_with(suffix) {
            return None;
        }

        let mut tokens: Vec<_> = command[prefix.len()..command.len()-1].split(" ").collect();
        // we don't actually care that it's addition in this case (below);
        // we just need to ensure that it's any Oper so things start correctly.
        let mut last_item = ProblemItem::Oper(Operation::Add);
        let mut ret = Vec::new();
        while tokens.len() > 0 {
            match last_item {
                ProblemItem::Oper(_) => {
                    // parse the current item as a value
                    let token = tokens.remove(0);
                    if let Ok(val) = isize::from_str_radix(token, 10) {
                        let piv = ProblemItem::Val(val);
                        ret.push(piv);
                        last_item = piv;
                    } else {
                        return None;
                    }
                }
                ProblemItem::Val(_) => {
                    // parse the current item as an operation
                    let token = tokens.remove(0);
                    let pio = match token {
                        "plus" => ProblemItem::Oper(Operation::Add),
                        "minus" => ProblemItem::Oper(Operation::Sub),
                        "multiplied" => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            // get rid of "by"
                            tokens.remove(0);
                            ProblemItem::Oper(Operation::Mul)
                        }
                        "divided" => {
                            if tokens.len() == 0 {
                                return None;
                            }
                            // get rid of "by"
                            tokens.remove(0);
                            ProblemItem::Oper(Operation::Div)
                        }
                        _ => return None,
                    };
                    ret.push(pio);
                    last_item = pio;
                }
            }
        }
        Some(ret)
    }

}
