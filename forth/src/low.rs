use crate::{Error, Result, Value};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Primitive {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Num(Value),
}

impl FromStr for Primitive {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "+" => Primitive::Add,
            "-" => Primitive::Sub,
            "*" => Primitive::Mul,
            "/" => Primitive::Div,
            "dup" => Primitive::Dup,
            "drop" => Primitive::Drop,
            "swap" => Primitive::Swap,
            "over" => Primitive::Over,
            _ => Primitive::Num(s.parse().map_err(|_| Error::InvalidWord)?),
        })
    }
}

pub(crate) fn evaluate(stack: &mut Vec<Value>, primitive: Primitive) -> Result {
    let mut pop = || stack.pop().ok_or(Error::StackUnderflow);
    match primitive {
        Primitive::Add => {
            let (r, l) = (pop()?, pop()?);
            stack.push(l + r);
        }
        Primitive::Sub => {
            let (r, l) = (pop()?, pop()?);
            stack.push(l - r);
        }
        Primitive::Mul => {
            let (r, l) = (pop()?, pop()?);
            stack.push(l * r);
        }
        Primitive::Div => {
            let (r, l) = (pop()?, pop()?);
            stack.push(l / r);
        }
        Primitive::Dup => {
            let v = pop()?;
            stack.push(v);
            stack.push(v);
        }
        Primitive::Drop => {
            pop()?;
        }
        Primitive::Swap => {
            (stack.len() >= 2)
                .then(|| {
                    let size = stack.len();
                    stack.swap(size - 1, size - 2)
                })
                .ok_or(Error::StackUnderflow)?;
        }
        Primitive::Over => {
            (stack.len() >= 2)
                .then(|| stack.push(stack[stack.len() - 2]))
                .ok_or(Error::StackUnderflow)?;
        }
        Primitive::Num(n) => stack.push(n),
    }
    Ok(())
}
