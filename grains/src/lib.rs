pub fn square(s: u32) -> u64 {
    match s {
        1...64 => 2u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    // in the future, we could use (1...64), but inclusive range syntax
    // is still experimental.
    (1..65).map(square).sum()
}
