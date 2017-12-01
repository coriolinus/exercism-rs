//! Modular math
//!
//! Perform multiplication and exponentiation with modulo, without overflow.
//! The other option was to use num_bigint; this should be more efficient.
//!
//! Cribbed from [wikipedia][wiki].
//!
//! [wiki]: https://en.wikipedia.org/wiki/Modular_arithmetic#Example_implementations

/// Compute `(x.pow(y)) % m` without overflow or bigints
///
/// https://en.wikipedia.org/wiki/Modular_arithmetic#Example_implementations
pub fn pow_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut r = 1;
    while b > 0 {
        if b % 2 == 1 {
            r = mul_mod(r, a, m)
        }
        b >>= 1;
        a = mul_mod(a, a, m);
    }
    r
}

/// Compute `(x * y) % m` without overflow or bigints
///
/// https://stackoverflow.com/a/45924957/504550
pub fn mul_mod(mut x: u64, mut y: u64, m: u64) -> u64 {
    let msb = 0x8000_0000_0000_0000;
    let mut d = 0;
    let mp2 = m >> 1;
    x %= m;
    y %= m;

    if m & msb == 0 {
        for _ in 0..64 {
            d = if d > mp2 { (d << 1) - m } else { d << 1 };
            if x & msb != 0 {
                d += y;
            }
            if d >= m {
                d -= m;
            }
            x <<= 1;
        }
        d
    } else {
        for _ in 0..64 {
            d = if d > mp2 {
                d.wrapping_shl(1).wrapping_sub(m)
            } else {
                // the case d == m && x == 0 is taken care of
                // after the end of the loop
                d << 1
            };
            if x & msb != 0 {
                let (mut d1, overflow) = d.overflowing_add(y);
                if overflow {
                    d1 = d1.wrapping_sub(m);
                }
                d = if d1 >= m { d1 - m } else { d1 };
            }
            x <<= 1;
        }
        if d >= m { d - m } else { d }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_mod() {
        let half = 1 << 16;
        let max = ::std::u64::MAX;

        assert_eq!(mul_mod(0, 0, 2), 0);
        assert_eq!(mul_mod(1, 0, 2), 0);
        assert_eq!(mul_mod(0, 1, 2), 0);
        assert_eq!(mul_mod(1, 1, 2), 1);
        assert_eq!(mul_mod(42, 1, 2), 0);
        assert_eq!(mul_mod(1, 42, 2), 0);
        assert_eq!(mul_mod(42, 42, 2), 0);
        assert_eq!(mul_mod(42, 42, 42), 0);
        assert_eq!(mul_mod(42, 42, 41), 1);
        assert_eq!(mul_mod(1239876, 2948635, 234897), 163320);

        assert_eq!(mul_mod(1239876, 2948635, half), 18476);
        assert_eq!(mul_mod(half, half, half), 0);
        assert_eq!(mul_mod(half + 1, half + 1, half), 1);

        assert_eq!(mul_mod(max, max, max), 0);
        assert_eq!(mul_mod(1239876, 2948635, max), 3655941769260);
        assert_eq!(mul_mod(1239876, max, max), 0);
        assert_eq!(mul_mod(1239876, max - 1, max), max - 1239876);
        assert_eq!(mul_mod(max, 2948635, max), 0);
        assert_eq!(mul_mod(max - 1, 2948635, max), max - 2948635);
        assert_eq!(mul_mod(max - 1, max - 1, max), 1);
        assert_eq!(mul_mod(2, max / 2, max - 1), 0);
    }
}
