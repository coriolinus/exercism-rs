use std::char;
use std::fmt;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate try_opt;
extern crate itertools;
use itertools::Itertools;
extern crate permutohedron;
use permutohedron::Heap;

pub type Mapping = HashMap<char, u8>;
type LetterSet = HashSet<char>;

struct Word {
    chars: Vec<char>,
}

impl Word {
    fn value(&self, map: &Mapping) -> Option<usize> {
        self.chars
            .iter()
            .map(|c| map.get(c).map_or(None, |d| char::from_digit(*d as u32, 10)))
            .collect::<Option<String>>()
            .map_or(None, |s| s.parse().ok())
    }

    fn from_str(input: &str) -> Option<Word> {
        if input.chars().any(|c| !c.is_alphabetic()) {
            None
        } else {
            Some(Word { chars: input.chars().collect() })
        }
    }

    fn letters(&self) -> LetterSet {
        self.chars.iter().cloned().collect()
    }

    fn starts_with_zero(&self, map: &Mapping) -> bool {
        !self.chars.is_empty() && *map.get(self.chars.first().unwrap()).unwrap_or(&0) == 0
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ws = self.chars.iter().collect::<String>();
        write!(f, "{}", ws)
    }
}

#[derive(PartialEq, Eq)]
enum TokenType {
    Plus,
    Equals,
    Word,
}

struct Equation {
    left: Vec<Word>,
    right: Vec<Word>,
    letters: Vec<char>,
}

impl Equation {
    fn parse(input: &str) -> Option<Equation> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut letters = LetterSet::new();
        let mut prev_token_type = None;

        let mut encountered_equals = false;

        for token in input.split_whitespace() {
            let token_type = match token {
                "+" => Some(TokenType::Plus),
                "==" => {
                    if encountered_equals {
                        return None; // can't solve simultaneous equations
                    }
                    encountered_equals = true;
                    Some(TokenType::Equals)
                }
                other => {
                    let word = try_opt!(Word::from_str(other));
                    for letter in word.letters() {
                        letters.insert(letter);
                    }
                    if encountered_equals {
                        right.push(word);
                    } else {
                        left.push(word);
                    }
                    Some(TokenType::Word)
                }
            };
            if token_type == prev_token_type {
                return None; // invalid equation
            }
            prev_token_type = token_type;
        }

        if letters.len() > 10 {
            // can't uniquely map each letter to a digit
            return None;
        }

        Some(Equation {
            left: left,
            right: right,
            letters: letters.iter().cloned().collect(),
        })
    }

    fn generate_mapping(&self, permutation: &[u8]) -> Mapping {
        self.letters.iter().cloned().zip_eq(permutation).map(|(k, &v)| (k, v)).collect()
    }

    fn solve(&self) -> Option<Mapping> {
        let mut solution = None;

        for mut digit_set in (0..10).combinations(self.letters.len()) {
            let heap = Heap::new(&mut digit_set);
            for permutation in heap {
                let map = self.generate_mapping(&permutation);
                if self.evaluate(&map) == Some(true) {
                    if solution.is_some() {
                        // reject puzzles with multiple solutions
                        return None;
                    } else {
                        solution = Some(map);
                    }
                }
            }
        }

        solution
    }

    fn evaluate(&self, map: &Mapping) -> Option<bool> {
        Some(!self.left.iter().chain(self.right.iter()).any(|word| word.starts_with_zero(map)) &&
             try_opt!(self.left.iter().map(|word| word.value(map)).collect::<Option<Vec<_>>>())
            .iter()
            .sum::<usize>() ==
             try_opt!(self.right.iter().map(|word| word.value(map)).collect::<Option<Vec<_>>>())
            .iter()
            .sum::<usize>())
    }
}

pub fn solve(puzzle: &str) -> Option<Mapping> {
    let equation = try_opt!(Equation::parse(puzzle));
    equation.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_map(k_v: &[(char, u8)]) -> Mapping {
        k_v.iter().cloned().collect()
    }

    #[test]
    fn test_word_value() {
        let word = Word::from_str("bob").unwrap();
        let map = make_map(&[('b', 1), ('o', 0)]);
        assert_eq!(word.value(&map).unwrap(), 101);

        let word = Word::from_str("marley").unwrap();
        let map = make_map(&[('m', 6), ('a', 5), ('r', 4)]);
        assert!(word.value(&map).is_none());
        let map = make_map(&[('m', 6), ('a', 5), ('r', 4), ('l', 3), ('e', 2), ('y', 1), ('o', 0)]);
        assert_eq!(word.value(&map).unwrap(), 654321);
    }
}
