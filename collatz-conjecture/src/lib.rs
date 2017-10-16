pub struct Collatz {
    n: u64,
    reached_1: bool,
}

impl Collatz {
    pub fn new(n: u64) -> Result<Collatz, &'static str> {
        if n == 0 {
            Err("Collatz sequence must begin with non-zero")
        } else {
            Ok(Collatz {
                n: n,
                reached_1: n == 1,
            })
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_1 {
            return None;
        }
        match self.n % 2 {
            0 => {
                self.n /= 2;
            }
            1 => {
                self.n = (3 * self.n) + 1;
            }
            _ => unreachable!(),
        }
        if self.n == 1 {
            self.reached_1 = true;
        }
        Some(self.n)
    }
}

// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    Ok(Collatz::new(n)?.count() as u64)
}
