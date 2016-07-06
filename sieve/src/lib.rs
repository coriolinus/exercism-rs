/// Sieve of Eratosthenes
/// generator implementation
pub fn primes_up_to(limit: u64) -> Vec<u64> {
    PrimesIter::new().take_while(|p| p <= &limit).collect()
}

pub struct PrimesIter {
    counter: u64,
    primes: Vec<u64>,
}

impl PrimesIter {
    pub fn new() -> Self {
        PrimesIter {
            counter: 2,
            primes: Vec::new(),
        }
    }

    /// Test if a given number is prime, given the current state.
    /// Not general purpose! Depends on having a complete list of all primes up to sqrt(n).
    fn is_prime(&self, n: u64) -> bool {
        for prime in self.primes.iter().take_while(|&p| p * p <= n) {
            if n % prime == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimesIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut counter = self.counter;
        while ! self.is_prime(counter) {
            counter += 1;
        }
        self.primes.push(counter);

        self.counter = counter + 1;
        Some(counter)
    }
}
