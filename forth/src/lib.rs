#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;

lazy_static! {
    static ref NON_WORD: Regex = Regex::new(r"[\s\pC]+").unwrap();
    static ref NUMBER: Regex = Regex::new(r"-?\d+").unwrap();
    static ref KEYWORDS: HashSet<String> = {
        let mut hs = HashSet::with_capacity(10);
        // nope, actual keywords aren't protected in Forth, apparently
        for kw in vec![":", ";", "+", "*", "-", "/"] {
            hs.insert(kw.to_string());
        }
        hs.shrink_to_fit();
        hs
    };
}

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<i32>,
    state: State,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(PartialEq, Eq, Clone)]
enum State {
    Normal,
    GetWord,
    CollectWordInstructions(Rc<String>),
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            state: State::Normal,
            words: HashMap::new(),
        }
    }

    pub fn format_stack(&self) -> String {
        self.stack.iter().map(i32::to_string).collect::<Vec<_>>().join(" ")
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        // can't tokenize with a simple input.split_whitespace() because
        // the chars \u{0} and \u{1} from the test case aren't in the
        // Unicode `White_Space` class
        for token in NON_WORD.split(input) {
            try!(self.eval_token(token));
        }
        if self.state == State::Normal {
            Ok(())
        } else {
            Err(Error::InvalidWord)
        }
    }

    fn eval_token(&mut self, token: &str) -> ForthResult {
        // Sharp-eyed readers will say "Wait a second! You're cloning a type, here,
        // which might contain a String! Isn't that potentially expensive?"
        //
        // I wasn't a huge fan of it either, which is why I wrapped the string in
        // an Rc smart pointer. This gives shared thread-local reference-counted
        // references which clone extremely cheaply: after all, you're just copying
        // the pointer, not the data.
        //
        // Cloning is strictly necessary here:
        //
        // Matching against simple self.state is a non-starter, because self is
        // borrowed in the context of this function.
        //
        // If we match against &self.state, we can't modify self.state within any given
        // match arm because self is already immutably borrowed. Matching against
        // &mut self.state seems to work at first, because you can use the
        // `state @ &mut State::Normal` syntax to get a local binding to the state, and
        // then assign with `*state = State::GetWord`. Unfortunately, that mutably
        // borrows all of self, meaning that you can't do self.pop() anymore.
        // Non-lexical borrows would fix this, but those are a far-future feature
        // at the moment.
        //
        // Before getting to this point, I had an idea to wrap either all of self
        // or at least self.state in a Cow, but I couldn't figure out how to make
        // it work; I think I may have been using it wrong.
        let value = match (self.state.clone(), token.to_uppercase().as_str()) {
            // deal with setting up new words using states
            (State::Normal, ":") => {
                self.state = State::GetWord;
                return Ok(());
            }
            (State::GetWord, word) => {
                if KEYWORDS.contains(word) || NUMBER.is_match(word) {
                    return Err(Error::InvalidWord);
                }
                self.words.insert(word.to_string(), Vec::new());
                self.state = State::CollectWordInstructions(Rc::new(word.to_string()));
                return Ok(());
            }
            (State::CollectWordInstructions(_), ";") => {

                self.state = State::Normal;
                return Ok(());
            }
            (State::CollectWordInstructions(word), token) => {

                self.words.get_mut((*word).as_str()).unwrap().push(token.to_string());
                return Ok(());
            }
            // all states from here on are State::Normal with a regular token
            // numbers and arithmetic
            (_, token) if NUMBER.is_match(token) => {
                try!(i32::from_str(token).map_err(|_| Error::InvalidWord))
            }
            // Can't wait for the ? operator
            // to make this next section a little prettier
            (_, "+") => try!(self.pop()) + try!(self.pop()),
            (_, "*") => try!(self.pop()) * try!(self.pop()),
            (_, "-") => (-try!(self.pop())) + try!(self.pop()),
            (_, "/") => {
                let dividend = try!(self.pop());
                if dividend == 0 {
                    return Err(Error::DivisionByZero);
                }
                try!(self.pop()) / dividend
            }
            // look for user-defined words before falling back on builtins
            (_, word) if self.words.get(word).is_some() => {
                // we have to clone the list of instructions, because as far as the
                // borrow checker knows, we could define new instructions within
                // that list, or (more relevantly) modify or overwrite the current
                // instruction list. Cloning it ensures that the list we're iterating
                // over doesn't change.
                for instruction in try!(self.words.get(word).ok_or(Error::UnknownWord)).clone() {
                    try!(self.eval_token(&instruction));
                }
                return Ok(());
            }
            // fundamental operations
            (_, "DUP") => *try!(self.last()),
            (_, "DROP") => {
                try!(self.pop());
                return Ok(());
            }
            (_, "SWAP") => {
                let a = try!(self.pop());
                let b = try!(self.pop());
                self.stack.push(a);
                b
            }
            (_, "OVER") => {
                let a = try!(self.pop());
                let b = *try!(self.last());
                self.stack.push(a);
                b
            }
            // when all else fails, give up
            (_, _) => return Err(Error::UnknownWord),
        };
        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<i32, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn last(&self) -> Result<&i32, Error> {
        self.stack.last().ok_or(Error::StackUnderflow)
    }
}
