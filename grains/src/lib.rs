pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!("Square must be between 1 and 64")
    }
    if s > 64 {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // in the future, we could use (1...64), but inclusive range syntax
    // is still experimental.
    (1..65).map(square).sum()
}
