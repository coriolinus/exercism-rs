#[derive(Debug)]
pub enum LSPError {
    RequestedSequenceLongerThanInput,
    InputSequenceContainsNonDigits,
}

/// Compute the largest sequential product of a string of digits
///
/// This is a two-pass solution: first, we get ourselves a vector
/// of digits. Then, we maintain a running product as we iterate
/// over the digits, resetting when we encounter a zero. This solution
/// should be efficient over very long digit strings.
pub fn lsp(seq: &str, n: usize) -> Result<usize, LSPError> {
    let digits = seq.chars()
        .map(|c| c.to_digit(10).map(|d| d as usize))
        .collect::<Option<Vec<usize>>>()
        .ok_or(LSPError::InputSequenceContainsNonDigits)?;
    if n > digits.len() {
        return Err(LSPError::RequestedSequenceLongerThanInput);
    }

    let mut product = digits.iter().take(n).product();
    let mut greatest_product = product;

    for (idx, (head, tail)) in digits.iter().skip(n).zip(digits.iter()).enumerate() {
        if *tail == 0 {
            // we have to reset the product for the new run
            product = digits.iter().skip(idx + 1).take(n).product();
        } else {
            product = product * head / tail;
        }
        if product > greatest_product {
            greatest_product = product;
        }
    }
    Ok(greatest_product)
}
