// We end up using u32s here because that's the type returned by char::to_digit

fn luhn_double(d: u32) -> u32 {
    assert!(d < 10);
    let d2 = 2 * d;
    if d2 > 9 { d2 - 9 } else { d2 }
}

/// Test if the provided string is a valid luhn number.
///
/// Though the performance here is O(n), it's technically O(3n),
/// where I'd prefer it to be O(1n). Unfortunately, I can't figure
/// out a way to do that properly. My instinct is to i.e. do something
/// like replace the map statement below with
///     .map(|c| match c.to_digit(10) {
///         Some(d) => d,
///         None => return false,
///     })
/// but that's a type error, because the branches from the match arms
/// are unequal, despite one of them being a return statement.
///
/// Likewise, given that we're already enumerating this iterator,
/// it would be good to be able to get at its last value without
/// re-iterating over the string. However, the type system doesn't
/// believe that the iterator chain here implements Clone, though
/// I can't see why not.
pub fn is_valid(n: &str) -> bool {
    if !(n.chars().filter(|c| *c != ' ').all(|v| v.is_digit(10)) &&
         n.chars().filter(|c| *c != ' ').count() > 1) {
        return false;
    }
    n.chars()
        .rev()
        .filter(|c| *c != ' ')
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(idx, d)| if idx % 2 == 0 { d } else { luhn_double(d) })
        .sum::<u32>() % 10 == 0
}
