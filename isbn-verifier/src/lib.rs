
#[macro_use]
extern crate try_opt;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if let Some(isbn) = Isbn::try_from(isbn) {
        isbn.is_valid()
    } else {
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Isbn {
    digits: [u8; 10],
}

impl Isbn {
    fn try_from_chars<I: IntoIterator<Item = char>>(input: I) -> Option<Isbn> {
        let mut digits = [0; 10];
        for (idx, ch) in input.into_iter().enumerate() {
            if idx == 9 && ch == 'X' {
                digits[idx] = 10;
            } else {
                digits[idx] = try_opt!(ch.to_digit(10)) as u8;
            }
        }
        Some(Isbn { digits: digits })
    }

    pub fn try_from(input: &str) -> Option<Isbn> {
        if input.len() == 10 {
            Isbn::try_from_chars(input.chars())
        } else if input.len() == 13 {
            Isbn::try_from_chars(
                input
                    .chars()
                    .enumerate()
                    .filter(|&(idx, _)| idx != 1 && idx != 5 && idx != 11)
                    .map(|(_, ch)| ch),
            )
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        self.digits.iter().rev().enumerate().fold(
            0,
            |acc, (idx, &d)| {
                acc + ((idx + 1) * (d as usize))
            },
        ) % 11 == 0
    }
}
