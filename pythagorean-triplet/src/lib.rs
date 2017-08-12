#[macro_use]
extern crate try_opt;

/// Find the answer asked for by README.md
///
/// WTF is up with this interface, though? This is the opposite of modular!
pub fn find() -> Option<u64> {
    let (a, b, c) = try_opt!(find_triplet_with_sum(1000));
    Some(a + b + c)
}

/// Find a Pythagorean Triplet with a given sum
///
/// This is more what a function's interface should look like.
///
/// Anyway, per Wikipedia: "Together, the Stifel and Ozanam sequences produce all primitive
/// triples of the Plato and Pythagoras families respectively". This does leave out the
/// Fermat family, and perhaps others (?); my pure math knowledge is insufficient on this
/// point. However, this gives us a decent chance to generate the correct number.
pub fn find_triplet_with_sum(sum: u64) -> Option<(u64, u64, u64)> {
    find_triplet_with_sum_using(sum, stifel).or(find_triplet_with_sum_using(sum, ozanam))
}

fn find_triplet_with_sum_using<F>(sum: u64, method: F) -> Option<(u64, u64, u64)>
where
    F: Fn(u64) -> (u64, u64, u64),
{
    let mut i = 0;
    loop {
        i += 1;
        let (a, b, c) = method(i);
        let method_sum = a + b + c;

        if method_sum == sum {
            return Some((a, b, c));
        }
        if method_sum > sum {
            return None;
        }
    }
}

/// Generate a number from the Stifel sequence
///
/// See https://en.wikipedia.org/wiki/Formulas_for_generating_Pythagorean_triples
fn stifel(n: u64) -> (u64, u64, u64) {
    let denominator = (2 * n) + 1;
    let numerator = (n * denominator) + n;
    let hypotenuse = numerator + 1;
    (denominator, numerator, hypotenuse)
}

#[cfg(test)]
pub fn t_stifel(n: u64) -> (u64, u64, u64) {
    stifel(n)
}

/// Generate a number from the Ozanam sequence
///
/// See https://en.wikipedia.org/wiki/Formulas_for_generating_Pythagorean_triples
fn ozanam(n: u64) -> (u64, u64, u64) {
    let denominator = (4 * n) + 4;
    let numerator = (4 * n) + 3 + (denominator * n);
    let hypotenuse = numerator + 2;
    (denominator, numerator, hypotenuse)
}

#[cfg(test)]
pub fn t_ozanam(n: u64) -> (u64, u64, u64) {
    ozanam(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stifel() {
        assert_eq!(t_stifel(1), (3, 4, 5));
        assert_eq!(t_stifel(2), (5, 12, 13));
        assert_eq!(t_stifel(3), (7, 24, 25));
        assert_eq!(t_stifel(4), (9, 40, 41));
    }

    #[test]
    fn test_ozanam() {
        assert_eq!(t_ozanam(1), (8, 15, 17));
        assert_eq!(t_ozanam(2), (12, 35, 37));
        assert_eq!(t_ozanam(3), (16, 63, 65));
        assert_eq!(t_ozanam(4), (20, 99, 101));
    }
}
