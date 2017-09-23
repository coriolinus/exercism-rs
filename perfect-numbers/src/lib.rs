#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(number: u64) -> Result<Classification, &'static str> {
    if number == 0 {
        return Err("Number must be positive");
    }
    let aliquot_sum: u64 = (1..((number / 2) + 1)).filter(|n| number % n == 0).sum();
    {
        use std::cmp::Ordering::*;
        use Classification::*;
        Ok(match aliquot_sum.cmp(&number) {
            Less => Deficient,
            Equal => Perfect,
            Greater => Abundant,
        })
    }
}
