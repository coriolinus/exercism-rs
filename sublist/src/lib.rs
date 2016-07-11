#[derive(Debug, PartialEq)]
pub enum Comparison {
    Sublist,
    Superlist,
    Equal,
    Unequal,
}

impl Comparison {
    pub fn invert(self) -> Self {
        match self {
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            comp => comp, // other matches are returned unchanged
        }
    }
}

pub fn sublist<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() > b.len() {
        sublist(b, a).invert()
    } else if a.len() == b.len() {
        if a == b {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else {
        // a is a list shorter than b with at least 1 element.
        // Is it a sublist of b?
        // I dunno, let's try recursive descent to find out.
        // That is, let's try reducing the length of B from either side until
        // we get to the length of A, at which point we'll know.
        let drop_first = {
            let (_, rest) = b.split_first().unwrap();
            sublist(a, rest)
        };
        let drop_last = {
            let (_, rest) = b.split_last().unwrap();
            sublist(a, rest)
        };
        match (drop_first, drop_last) {
            // if either arm returns Equal or Sublist, it's a sublist.
            // this is because there exists a path which reduces B to A.
            // Therefore, the only case in which it's Unequal is if both
            // recursive arms are unequal.
            (Comparison::Unequal, Comparison::Unequal) => Comparison::Unequal,
            (_, _) => Comparison::Sublist,
        }
    }
}
