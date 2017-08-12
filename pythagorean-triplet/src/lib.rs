#[macro_use]
extern crate try_opt;

/// Find the answer asked for by README.md
///
/// WTF is up with this interface, though? This is the opposite of modular!
pub fn find() -> Option<u64> {
    let (a, b, c) = try_opt!(find_triplet_with_sum(1000));
    Some(a * b * c)
}

pub type Triplet = (u64, u64, u64);

fn is_pythagorean(t: Triplet) -> bool {
    t.0 * t.0 + t.1 * t.1 == t.2 * t.2
}

/// Find a Pythagorean Triplet with a given sum
///
/// This is more what a function's interface should look like.
pub fn find_triplet_with_sum(sum: u64) -> Option<Triplet> {
    Triplets::new(sum).filter(|&t| is_pythagorean(t)).next()
}

/// Iterator over possible natural triplets which sum to the desired value
///
/// `c` is always equal to `total - (a + b)`.
/// The property `b >= a` is always observed.
/// The property `c > b` is always observed.
struct Triplets {
    total: u64,
    a: u64,
    b: u64,
}

impl Triplets {
    fn new(sum: u64) -> Triplets {
        Triplets {
            total: sum,
            a: 1,
            b: 0,
        }
    }

    fn obeys_properties(&self) -> bool {
        self.a <= self.b && self.a + self.b < self.total && self.b < self.c()
    }

    // be careful when using this: if you don't already know that a and b are valid,
    // it's possible to get an underflow
    fn c(&self) -> u64 {
        self.total - (self.a + self.b)
    }
}

impl Iterator for Triplets {
    type Item = Triplet;
    fn next(&mut self) -> Option<Triplet> {
        // can we increment b?
        self.b += 1;
        if self.obeys_properties() {
            return Some((self.a, self.b, self.c()));
        }
        // if not, can we increment a?
        self.a += 1;
        self.b = self.a;
        if self.obeys_properties() {
            return Some((self.a, self.b, self.c()));
        }
        None
    }
}
