pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Err("inputs of different length");
    }

    Ok(
        a.chars().zip(b.chars())
            .map(|(ac, bc)| if ac == bc { 0 } else { 1 })
            .fold(0, std::ops::Add::add)
    )
}
