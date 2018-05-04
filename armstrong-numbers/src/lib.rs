pub fn is_armstrong_number(num: u32) -> bool {
    // unwrap is safe here because we know that this was just a number
    let digits = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    digits
        .iter()
        .map(|d| d.pow(digits.len() as u32))
        .sum::<u32>() == num
}
