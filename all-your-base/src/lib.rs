///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    let number = try!(from_base_n(number, from_base));
    Ok(to_base_n(number, to_base))
}

fn from_base_n(number: &[u32], base: u32) -> Result<usize, ()> {
    let mut num = 0;
    for (exp, digit) in number.iter().rev().enumerate() {
        if *digit >= base {
            return Err(());
        }
        num += (*digit * base.pow(exp as u32)) as usize
    }
    Ok(num)
}

fn to_base_n(mut number: usize, base: u32) -> Vec<u32> {
    // First, we find the minimum exponentiation of base which is greater than number.
    // This tells us what exponent to start with.
    let mut exponent = 0;
    while number >= (base as usize).pow(exponent) {
        exponent += 1;
    }

    let mut digits = Vec::with_capacity(exponent as usize + 1);

    // Then, count down the exponents, adding the appropriate digit for each.
    for exp in (0..exponent).rev() {
        // the appropriate digit can be derived by dividing the number by the positional value
        digits.push((number / (base as usize).pow(exp)) as u32);
        // now keep the remainder
        number = number % (base as usize).pow(exp);
    }

    digits
}
