#[macro_use]
extern crate enum_primitive_derive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate itertools;
extern crate num_traits;

pub use self::interval::{Interval, Intervals};
use self::note::Accidental;
pub use self::note::Note;
use failure::Error;
use std::str::FromStr;

pub mod interval;
pub mod note;

#[derive(Debug)]
pub struct Scale {
    tonic: Note,
    lean: Accidental,
    intervals: Intervals,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic: Note::from_str(tonic)?,
            lean: Accidental::from_tonic(tonic),
            intervals: Intervals::from_str(intervals)?,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(self.intervals.len());

        let mut note = self.tonic;
        for &interval in self.intervals.iter() {
            out.push(note.canonicalize(self.lean).to_string());
            note += interval;
        }

        out
    }
}
