use super::Mapping;
use permutator::copy::KPermutationIterator;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref DIGITS: Vec<u8> = (0..=9).collect();
}

/// precondition: `leading` is a subset of `chars`
pub fn generate_mappings(
    chars: &HashSet<char>,
    leading: HashSet<char>,
) -> impl Iterator<Item = Mapping> {
    let chars = {
        let mut chars: Vec<_> = chars.iter().copied().collect();
        chars.sort();
        chars
    };

    KPermutationIterator::new(&DIGITS, chars.len())
        .map(move |digits| chars.iter().copied().zip(digits).collect::<Mapping>())
        .filter(move |mapping| {
            leading.iter().all(|leading_char| {
                *mapping
                    .get(leading_char)
                    .expect("precondition: leading is a subset of chars")
                    != 0
            })
        })
}
