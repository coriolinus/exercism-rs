pub fn is_armstrong_number(num: u32) -> bool {
    // unwrap is safe here because we know that this was just a number
    let digits = num.to_string();
    digits
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(digits.len() as u32))
        .sum::<u32>() == num
}
