use crate::{low::Primitive, Error};
use genawaiter::{rc::gen, yield_};
use std::collections::HashMap;

pub(crate) type Definitions = HashMap<String, Vec<Vec<Token>>>;

#[derive(Debug)]
pub(crate) enum Token {
    BeginDefinition,
    EndDefinition,
    Primitive(Primitive),
    Reference(String, usize),
}

impl Token {
    /// Construct a token from a string
    ///
    /// We can't use `std::str::FromStr`, even though it's directly applicable,
    /// because this method requires some context: the `Definitions`.
    fn from_str(word: &str, definitions: &Definitions) -> Result<Token, Error> {
        word.parse::<Primitive>()
            .map(Token::Primitive)
            .or_else(|_| match word {
                ":" => Ok(Token::BeginDefinition),
                ";" => Ok(Token::EndDefinition),
                _ => definitions
                    .get(word)
                    .map(|versions| Token::Reference(word.to_string(), versions.len()))
                    .ok_or(Error::UnknownWord),
            })
    }
}

#[derive(Debug)]
enum Mode {
    Normal,
    ExpectName,
    Definition(String, Vec<Token>),
}

/// Evaluate an input string, emitting an iterator of either primitives or errors as they arise.
pub(crate) fn evaluate<'a>(
    input: &'a str,
    definitions: &'a mut Definitions,
) -> impl 'a + Iterator<Item = Result<Primitive, Error>> {
    gen!({
        let mut mode = Mode::Normal;

        for word in input.split_whitespace() {
            let word = word.to_lowercase();

            // we want to parse the token in all cases _except_ when the mode is ExpectName.
            // this is because token parsing involves a little validation: if the token doesn't refer
            // to a word already in the definitions, it'll fail. ExpectName is when we
            let token = match mode {
                Mode::Normal | Mode::Definition(..) => {
                    match Token::from_str(&word, &definitions) {
                        Ok(token) => token,
                        Err(err) => {
                            yield_!(Err(err));
                            continue;
                        }
                    }
                }
                Mode::ExpectName => {
                    // we need some token here, but it's arbitrary and ignored.
                    Token::EndDefinition
                }
            };

            match mode {
                Mode::Normal => {
                    // generator syntax in Rust is still kind of clunky, particularly here:
                    // what I really want is a Python-ish `yield from eval_token_normal(...)`
                    for item in eval_token_normal(&token, &mut mode, &definitions) {
                        yield_!(item);
                    }
                }
                Mode::ExpectName => mode = Mode::Definition(word.to_string(), Vec::new()),
                // invariant in definition handler: the only tokens which ever get
                // pushed to `subsequent` are primitives and (valid) references.
                Mode::Definition(_, ref mut subsequent) => match token {
                    Token::BeginDefinition => yield_!(Err(Error::InvalidWord)),
                    Token::Primitive(..) | Token::Reference(..) => subsequent.push(token),
                    Token::EndDefinition => {
                        if let Mode::Definition(word, definition) =
                            std::mem::replace(&mut mode, Mode::Normal)
                        {
                            definitions.entry(word).or_default().push(definition);
                        } else {
                            unreachable!("this is definitely the current mode; we just don't have a better way to extract its values");
                        }
                    }
                },
            }
        }
    }).into_iter()
}

/// Evaluate a token in normal mode.
///
/// This function is broken out so it can recursively call itself when
/// expanding custom words.
///
/// Its recursion would get weird if it weren't for the invariant in the definition handler;
/// all definitions are known to contain only primitives or valid references.
///
/// Note that it returns a `Box<dyn Iterator>` instead of simply `impl Iterator`. This works
/// around a rustc limitation: Rust has trouble with recursive opaque types unless you box them
/// up in this way.
fn eval_token_normal<'a>(
    token: &'a Token,
    mode: &'a mut Mode,
    definitions: &'a Definitions,
) -> Box<dyn 'a + Iterator<Item = Result<Primitive, Error>>> {
    Box::new(
        gen!({
            match token {
                Token::Primitive(primitive) => yield_!(Ok(*primitive)),
                Token::BeginDefinition => *mode = Mode::ExpectName,
                Token::EndDefinition => yield_!(Err(Error::InvalidWord)),
                Token::Reference(word, index) => {
                    let definition = &definitions[word][*index];
                    for token in definition {
                        for item in eval_token_normal(token, mode, definitions) {
                            yield_!(item);
                        }
                    }
                }
            }
        })
        .into_iter(),
    )
}
