pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    match a.len() == b.len() {
        false => Err("inputs of different length"),
        true => Ok(
            a.chars().zip(b.chars())
                .filter(|&(ac, bc)| ac != bc)
                .count()
        )
    }
}
