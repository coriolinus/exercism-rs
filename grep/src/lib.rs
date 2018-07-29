#[macro_use]
extern crate bitflags;

extern crate failure;

use failure::Error;

use std::fs::File;
use std::io::{BufRead, BufReader};

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
bitflags! {
    pub struct Flags: u8 {
        const LINE_NUMBERS = 1 << 0;
        const FILE_NAMES = 1 << 1;
        const CASE_INSENSITIVE = 1 << 2;
        const INVERT_MATCH = 1 << 3;
        const FULL_LINE = 1 << 4;
        const SKIP_LINE_TEXT = 1 << 5;
    }
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut out: Flags = Flags::empty();
        for &flag in flags {
            out |= match flag {
                "-n" => Flags::LINE_NUMBERS,
                "-l" => Flags::SKIP_LINE_TEXT,
                "-i" => Flags::CASE_INSENSITIVE,
                "-v" => Flags::INVERT_MATCH,
                "-x" => Flags::FULL_LINE,
                _ => continue,
            };
        }
        out
    }
}

fn write(flags: &Flags, file: &str, line_number: usize, line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    if flags.contains(Flags::FILE_NAMES) {
        out.push_str(file);
        if flags.contains(Flags::LINE_NUMBERS) || !flags.contains(Flags::SKIP_LINE_TEXT) {
            out.push(':');
        }
    }
    if flags.contains(Flags::LINE_NUMBERS) {
        out.push_str(&format!("{}", line_number));
        if !flags.contains(Flags::SKIP_LINE_TEXT) {
            out.push(':');
        }
    }
    if !flags.contains(Flags::SKIP_LINE_TEXT) {
        out.push_str(line);
    }
    out
}

fn grep_file(pattern: &str, flags: &Flags, file: &str) -> Result<Vec<String>, Error> {
    let mut out = Vec::new();
    let invert = flags.contains(Flags::INVERT_MATCH);
    let fp = File::open(file)?;
    for (idx, line) in BufReader::new(fp).lines().enumerate() {
        let line_number = idx + 1;
        let line = line?;
        let cline = if flags.contains(Flags::CASE_INSENSITIVE) {
            line.clone().to_lowercase()
        } else {
            line.clone()
        };

        if flags.contains(Flags::FULL_LINE) {
            if invert ^ (cline.trim() == pattern) {
                out.push(write(flags, file, line_number, &line));
            }
        } else {
            if invert ^ cline.contains(pattern) {
                out.push(write(flags, file, line_number, &line));
            }
        }
    }
    Ok(out)
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = if flags.contains(Flags::CASE_INSENSITIVE) {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };
    let mut flags = *flags; // copies a u8; cheap
    if files.len() > 1 {
        flags |= Flags::FILE_NAMES;
    }
    if files.len() == 1 && flags.contains(Flags::SKIP_LINE_TEXT) {
        flags |= Flags::FILE_NAMES
    }

    let mut out = Vec::new();
    for file in files {
        out.extend(grep_file(&pattern, &flags, file)?);
    }
    out.dedup();
    Ok(out)
}
