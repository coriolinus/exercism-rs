use interval::Interval;
use num_traits::{FromPrimitive, ToPrimitive};
use std::fmt;
use std::ops::AddAssign;
use std::str::FromStr;

pub const SEMITONES: i8 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
pub enum Semitone {
    A = 0,
    ASharp = 1,
    B = 2,
    C = 3,
    CSharp = 4,
    D = 5,
    DSharp = 6,
    E = 7,
    F = 8,
    FSharp = 9,
    G = 10,
    GSharp = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Primitive)]
pub enum Root {
    A = 0,
    B = 2,
    C = 3,
    D = 5,
    E = 7,
    F = 8,
    G = 10,
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accidental {
    Sharp,
    Flat,
}

impl Accidental {
    fn to_i8(&self) -> i8 {
        match *self {
            Accidental::Sharp => 1,
            Accidental::Flat => -1,
        }
    }

    pub fn from_tonic(tonic: &str) -> Accidental {
        match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
            | "d#" => Accidental::Sharp,
            _ => Accidental::Flat,
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Accidental::Sharp => '#',
                Accidental::Flat => 'b',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    tonic: Root,
    accidental: Option<Accidental>,
}

impl Note {
    pub fn canonicalize(&self, lean: Accidental) -> Note {
        let mut n: Note = Semitone::from(*self).into();
        if let Some(accidental) = n.accidental {
            if accidental != lean {
                if lean == Accidental::Flat {
                    n += Interval::HalfStep;
                    n.accidental = Some(Accidental::Flat);
                }
            }
        }
        n
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.tonic,
            self.accidental.map_or(String::new(), |a| a.to_string()),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Fail)]
pub enum ParseErr {
    #[fail(display = "invalid length")]
    InvalidLength,
    #[fail(display = "invalid tonic")]
    InvalidTonic,
    #[fail(display = "invalid accidental")]
    InvalidAccidental,
}

impl FromStr for Note {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lc = s.to_lowercase();
        let mut iter = lc.chars();

        let mut note = match iter.next() {
            Some(c) if 'a' <= c && 'g' >= c => Note {
                tonic: match c {
                    'a' => Root::A,
                    'b' => Root::B,
                    'c' => Root::C,
                    'd' => Root::D,
                    'e' => Root::E,
                    'f' => Root::F,
                    'g' => Root::G,
                    _ => return Err(ParseErr::InvalidTonic),
                },
                accidental: None,
            },
            Some(_) => return Err(ParseErr::InvalidTonic),
            None => return Err(ParseErr::InvalidLength),
        };

        match iter.next() {
            Some('b') => note.accidental = Some(Accidental::Flat),
            Some('#') => note.accidental = Some(Accidental::Sharp),
            Some(_) => return Err(ParseErr::InvalidAccidental),
            None => {}
        }

        if iter.next().is_some() {
            return Err(ParseErr::InvalidLength);
        }

        Ok(note)
    }
}

impl From<Semitone> for Note {
    fn from(s: Semitone) -> Self {
        Note {
            tonic: match s {
                Semitone::A | Semitone::ASharp => Root::A,
                Semitone::B => Root::B,
                Semitone::C | Semitone::CSharp => Root::C,
                Semitone::D | Semitone::DSharp => Root::D,
                Semitone::E => Root::E,
                Semitone::F | Semitone::FSharp => Root::F,
                Semitone::G | Semitone::GSharp => Root::G,
            },
            accidental: match s {
                Semitone::ASharp
                | Semitone::CSharp
                | Semitone::DSharp
                | Semitone::FSharp
                | Semitone::GSharp => Some(Accidental::Sharp),
                _ => None,
            },
        }
    }
}

impl From<Note> for Semitone {
    fn from(n: Note) -> Self {
        Semitone::from_i8(
            (SEMITONES + n.tonic.to_i8().unwrap() + n.accidental.map_or(0, |a| a.to_i8()))
                % SEMITONES,
        ).expect("must have valid semitone")
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = Semitone::from_i8(
            (SEMITONES + Semitone::from(*self).to_i8().unwrap() + rhs.to_i8().unwrap()) % SEMITONES,
        ).unwrap()
            .into();
    }
}
