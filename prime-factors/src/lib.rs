// Generate primes using our Sieve of Erasthenes code from earlier
// In principle, we could skip this and just divide by every natural number,
// but this should be more efficient as the input values get large.
extern crate sieve;
use sieve::PrimesIter;

pub fn factors(mut value: u64) -> Vec<u64> {
    let mut out = Vec::new();

    loop {
        let mut divisible_prime = None;
        for prime in PrimesIter::new().take_while(|p| p * p <= value) {
            if value % prime == 0 {
                out.push(prime);
                divisible_prime = Some(prime);
                break; // value changed, so let's restart our primes iterator
            }
        }
        if let Some(p) = divisible_prime {
            // we can only divide by the prime we found here, because
            // this operation mutates `value`. In the loop above, `value`
            // is borrowed immutably in the `take_while`.
            value /= p;
        } else {
            // no more primes <= the square root of the value; we're done
            // don't forget to add the current value to the return list
            if value > 1 {
                out.push(value);
            }
            break;
        }
    }

    out
}
