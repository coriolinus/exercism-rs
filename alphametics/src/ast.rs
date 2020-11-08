use std::collections::HashSet;
use std::str::FromStr;

use super::Mapping;

/// A term in the equation; a word
struct Term {
    /// word is stored in reverse order
    word: Vec<char>,
}

impl FromStr for Term {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars()
            .all(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
        {
            Ok(Term {
                word: s.chars().rev().collect(),
            })
        } else {
            Err("term input contained invalid chars".into())
        }
    }
}

impl Term {
    fn chars(&self) -> HashSet<char> {
        self.word.iter().copied().collect()
    }

    fn leading(&self) -> Option<char> {
        // remember, `self.word` is stored in reverse order
        self.word.last().copied()
    }

    /// precondition: all chars in `self.word` appear in `mapping`.
    ///
    /// panics if that precondition is falsified.
    fn value_assuming(&self, mapping: &Mapping) -> i64 {
        let mut value = 0;
        for (idx, ch) in self.word.iter().enumerate() {
            value += 10_i64.pow(idx as u32)
                * *mapping
                    .get(ch)
                    .expect("precondition: mapping contains all chars in word")
                    as i64;
        }
        value
    }
}

enum Operator {
    Plus,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            _ => Err(format!("unexpected operator: {}", s)),
        }
    }
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Plus => a + b,
        }
    }
}

#[derive(derive_more::From)]
enum Token {
    Term(Term),
    Operator(Operator),
}

impl Token {
    fn expect_term(&self) -> &Term {
        match self {
            Self::Term(term) => &term,
            _ => panic!("expected term"),
        }
    }

    fn expect_operator(&self) -> &Operator {
        match self {
            Self::Operator(operator) => &operator,
            _ => panic!("expected operator"),
        }
    }
}

struct Expr {
    tokens: Vec<Token>,
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut want_term = true;
        let mut tokens = Vec::new();

        for (idx, token) in s.split_whitespace().enumerate() {
            if want_term {
                match token.parse::<Term>() {
                    Ok(term) => tokens.push(term.into()),
                    Err(err) => Err(format!(
                        "parsing \"{}\":\n  token {} @ {}:\n    attempted Term parse:\n      {}",
                        s, token, idx, err
                    ))?,
                }
            } else {
                match token.parse::<Operator>() {
                    Ok(op) => tokens.push(op.into()),
                    Err(err) => Err(format!("parsing \"{}\":\n  token {} @ {}:\n    attempted Operator parse:\n      {}", s, token, idx, err))?,
                }
            }
            want_term = !want_term;
        }

        Ok(Expr { tokens })
    }
}

impl Expr {
    fn chars(&self) -> HashSet<char> {
        self.tokens.iter().fold(HashSet::new(), |mut acc, token| {
            if let Token::Term(term) = token {
                acc.extend(term.chars());
            }
            acc
        })
    }

    fn leading(&self) -> HashSet<char> {
        let mut out = HashSet::new();
        for token in &self.tokens {
            if let Token::Term(term) = token {
                if let Some(l) = term.leading() {
                    out.insert(l);
                }
            }
        }
        out
    }

    /// precondition: all chars in all words in `self.tokens` appear in `mapping`
    /// precondition: `tokens` fits the pattern `(term (operator term)*)?`
    ///
    /// panics if a precondition is falsified.
    fn value_assuming(&self, mapping: &Mapping) -> i64 {
        let mut value = self
            .tokens
            .first()
            .map(|token| token.expect_term().value_assuming(mapping))
            .unwrap_or_default();

        for chunk in self.tokens[1..].chunks(2) {
            let operator = chunk[0].expect_operator();
            let term = chunk[1].expect_term();

            value = operator.apply(value, term.value_assuming(mapping));
        }

        value
    }
}

pub struct Equation {
    left: Expr,
    right: Expr,
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equal_idx = s.find("==").ok_or(String::from("failed to find \"==\""))?;
        let left = s[..equal_idx]
            .parse()
            .map_err(|err| format!("left: {}", err))?;
        let right = s[equal_idx + 2..]
            .parse()
            .map_err(|err| format!("right: {}", err))?;
        let eq = Equation { left, right };
        let chars = eq.chars();
        if chars.len() > 10 {
            Err(format!(
                "equation contains more than 10 distinct chars; unsolvable: {:?}",
                chars
            ))?;
        }
        Ok(eq)
    }
}

impl Equation {
    pub fn chars(&self) -> HashSet<char> {
        let mut out = self.left.chars();
        out.extend(self.right.chars());
        out
    }

    pub fn leading(&self) -> HashSet<char> {
        let mut out = self.left.leading();
        out.extend(self.right.leading());
        out
    }

    pub fn valid_assuming(&self, mapping: &Mapping) -> bool {
        self.left.value_assuming(mapping) == self.right.value_assuming(mapping)
    }
}
