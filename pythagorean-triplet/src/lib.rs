// this requires a nightly compiler, but the result is that we can compute
// the answer at compile-time, which is pretty cool
#![feature(const_fn)]

type N = u64;

const TARGET: N = 1000;

struct Triplet(N, N, N);

impl Triplet {
    const fn new(a: N, b: N) -> Triplet {
        Triplet(a, b, TARGET - (a + b))
    }

    const fn product(&self) -> N {
        self.0 * self.1 * self.2
    }

    const fn is_pythagorean(&self) -> bool {
        (self.0 * self.0) + (self.1 * self.1) == (self.2 * self.2)
    }

    const fn first() -> Triplet {
        Triplet(0, 0, TARGET)
    }

    const fn next(&self) -> Triplet {
        if self.1 >= TARGET {
            panic!("Couldn't find appropriate next triplet")
        }
        if self.0 + self.1 >= TARGET {
            Triplet::new(0, self.1 + 1)
        } else {
            Triplet::new(self.0 + 1, self.1)
        }
    }
}

pub const fn const_find() -> N {
    0
}

const ANSWER: N = const_find();

/// Find the answer asked for by README.md
///
/// WTF is up with this interface, though? This is the opposite of modular!
pub fn find() -> Option<N> {
    Some(ANSWER)
}
