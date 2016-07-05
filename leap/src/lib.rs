/// Return True if a given year is a leap year.
/// A year is a leap year:
/// on every year that is evenly divisible by 4
///  except every year that is evenly divisible by 100
///    unless the year is also evenly divisible by 400
pub fn is_leap_year(year: i16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
