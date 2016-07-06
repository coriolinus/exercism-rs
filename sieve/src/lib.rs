/// Sieve of Eratosthenes
/// classical implementation
pub fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut known_nonprime = vec![false; (limit + 1) as usize];

    for i in 2..limit + 1 {
        if !known_nonprime[i as usize] {
            primes.push(i);
            let mut counter = 2;
            while counter * i <= limit {
                known_nonprime[(counter * i) as usize] = true;
                counter += 1;
            }
        }
    }

    primes
}
