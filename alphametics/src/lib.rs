//! Note to reviewers: I'm well aware that stringly-typed errors are terrible, but I also know
//! that nobody is ever going to call this particular code as a library, so I decided not to bother
//! with the step of creating actual typed errors.

use std::collections::HashMap;

type Mapping = HashMap<char, u8>;

mod ast;
use ast::Equation;

mod mapping_gen;
use mapping_gen::generate_mappings;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equation: Equation = input.parse().map_err(|err| eprintln!("{}", err)).ok()?;
    let chars = equation.chars();
    let leading = equation.leading();

    for mapping in generate_mappings(&chars, leading) {
        if equation.valid_assuming(&mapping) {
            return Some(mapping);
        }
    }

    None
}
