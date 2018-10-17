use itertools::Itertools;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Fail)]
pub enum ParseErr {
    #[fail(display = "invalid interval")]
    InvalidInterval,
    #[fail(display = "wrong number of semitones")]
    WrongNumberOfSemitones,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
pub enum Interval {
    HalfStep = 1,
    WholeStep = 2,
    AugmentedFirst = 3,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Interval::*;
        write!(
            f,
            "{}",
            match self {
                HalfStep => "m",
                WholeStep => "M",
                AugmentedFirst => "A",
            }
        )
    }
}

impl FromStr for Interval {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Interval::*;
        match s {
            "m" => Ok(HalfStep),
            "M" => Ok(WholeStep),
            "A" => Ok(AugmentedFirst),
            _ => Err(ParseErr::InvalidInterval),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intervals(Vec<Interval>);

impl fmt::Display for Intervals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().join(""))
    }
}

impl FromStr for Intervals {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut semitones = Vec::with_capacity(s.len());

        for (i, c) in s.char_indices() {
            semitones.push(Interval::from_str(&s[i..i + c.len_utf8()])?);
        }

        if semitones.iter().take(12).map(|&i| i as u8).sum::<u8>() == 12 {
            Ok(Intervals(semitones))
        } else {
            Err(ParseErr::WrongNumberOfSemitones)
        }
    }
}

impl Deref for Intervals {
    type Target = Vec<Interval>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_chromatic() {
        assert!("mmmmmmmmmmmm".parse::<Intervals>().is_ok());
    }

    #[test]
    fn test_parse_major() {
        assert!("MMmMMMm".parse::<Intervals>().is_ok());
    }

    #[test]
    fn test_parse_minor() {
        assert!("MmMMmMM".parse::<Intervals>().is_ok());
    }
}
