extern crate sieve;
use sieve::PrimesIter;

pub fn nth(n: usize) -> Result<u64, &'static str> {
    match n {
        0 => Err("No such thing as 0th prime"),
        n => Ok(PrimesIter::new().skip(n - 1).next().unwrap()),
    }
}
