mod separate {
    use itertools::Itertools;
    use std::iter::FromIterator;

    pub trait Separate<'a, I, T, O>
    where
        I: IntoIterator<Item = T>,
        T: Copy,
        O: FromIterator<T>,
    {
        /// Separate a stream into groups, inserting a copy of T between each.
        /// Then collect it into a String.
        ///
        /// This is a fused iterator.
        fn separate(self, group_sep: T, group_size: usize) -> O;
    }

    impl<'a, I, T, O> Separate<'a, I, T, O> for I
    where
        I: 'a + IntoIterator<Item = T>,
        <I as IntoIterator>::IntoIter: 'a,
        T: 'a + Copy + PartialEq,
        O: FromIterator<T>,
    {
        fn separate(self, group_sep: T, group_size: usize) -> O {
            self.into_iter()
                .fuse()
                .chunks(group_size)
                .into_iter()
                .map(|chunk| {
                    let d: Box<dyn Iterator<Item = T>> = Box::new(chunk);
                    d
                })
                .interleave_shortest(std::iter::repeat(std::iter::once(group_sep)).map(|cyc| {
                    let d: Box<dyn Iterator<Item = T>> = Box::new(cyc);
                    d
                }))
                .flatten()
                .with_position()
                .filter_map(move |pos| {
                    use itertools::Position::*;
                    match pos {
                        Only(c) => Some(c),
                        First(c) => Some(c),
                        Middle(c) => Some(c),
                        Last(c) if c != group_sep => Some(c),
                        _ => None,
                    }
                })
                .collect()
        }
    }
}

use separate::Separate;

fn transpose(c: char) -> Option<char> {
    if !c.is_ascii_alphanumeric() {
        return None;
    }
    Some(if c.is_ascii_alphabetic() {
        let c = (c as u8) | (1 << 5); // lowercase ascii value
        (b'z' - (c - b'a')) as char
    } else {
        c
    })
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars().flat_map(transpose).separate(' ', 5)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().flat_map(transpose).collect()
}
