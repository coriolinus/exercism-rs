
/// The sum of the squares of the first ten natural numbers is,
///
/// ``` text
///     1**2 + 2**2 + ... + 10**2 = 385
/// ```
///
/// ```
/// # use difference_of_squares::sum_of_squares;
/// assert_eq!(sum_of_squares(10), 385);
/// ```
pub fn sum_of_squares(n: u64) -> u64 {
    // the below is unstable as of now
    // (1..n).map(|i| i^2).sum()
    (1..n+1).map(|i| i.pow(2)).fold(0, std::ops::Add::add)
}

/// The square of the sum of the first ten natural numbers is,
///
/// ``` text
///    (1 + 2 + ... + 10)**2 = 55**2 = 3025
/// ```
///
/// ```
/// # use difference_of_squares::square_of_sum;
/// assert_eq!(square_of_sum(10), 3025);
/// ```
pub fn square_of_sum(n: u64) -> u64 {
    (1..n+1).fold(0, std::ops::Add::add).pow(2)
}

/// Hence the difference between the square of the sum of the first
/// ten natural numbers and the sum of the squares is 2640:
///
/// ``` text
///     3025 - 385 = 2640
/// ```
///
/// ```
/// # use difference_of_squares::difference;
/// assert_eq!(difference(10), 2640)
/// ```
pub fn difference(n: u64) -> u64 {
    square_of_sum(n) - sum_of_squares(n)
}
