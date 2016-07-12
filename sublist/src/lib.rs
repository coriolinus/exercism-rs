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

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() > b.len() {
        sublist(b, a).invert()
    } else if a.len() == b.len() {
        if a == b {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if a.len() == 0 {
        Comparison::Sublist
    } else {
        // a is a list shorter than b
        // Is it a sublist of b?
        // OK, time to break out the Algorithms
        if kmp_search(a, b).is_some() {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    }
}

/// Knuth-Morris-Pratt substring search algorithm
/// largely cribbed here from the pseuducode at
/// https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm#Description_of_pseudocode_for_the_search_algorithm
#[allow(non_snake_case)]
pub fn kmp_search<T: PartialEq>(W: &[T], S: &[T]) -> Option<usize> {
    let mut m = 0;
    let mut i: isize = 0;
    let T = kmp_table(W);

    while ((m + i) as usize) < S.len() {
        if W[i as usize] == S[(m + i) as usize] {
            if i == (W.len() - 1) as isize {
                return Some(m as usize);
            }
            i += 1;
        } else {
            if T[i as usize] > -1 {
                m += i - T[i as usize];
                i = T[i as usize];
            } else {
                m += 1;
                i = 0;
            }
        }
    }
    None
}

/// Knuth-Morris-Pratt substring table-builder
/// https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm#Description_of_pseudocode_for_the_table-building_algorithm
#[allow(non_snake_case)]
fn kmp_table<C: PartialEq>(W: &[C]) -> Vec<isize> {
    let mut T: Vec<isize> = std::iter::repeat(0).take(W.len()).collect();
    T[0] = -1;

    let mut pos: usize = 2;
    let mut cnd: usize = 0;

    while pos < W.len() {
        if W[pos-1] == W[cnd] {
            T[pos] = cnd as isize + 1;
            cnd += 1;
            pos += 1;
        } else if cnd > 0 {
            cnd = T[cnd] as usize
        } else {
            T[pos] = 0;
            pos += 1;
        }
    }

    T
}
