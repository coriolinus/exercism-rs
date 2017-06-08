extern crate luhn;
use luhn::is_valid;

pub trait Luhn: ToString {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(&self.to_string())
    }
}
