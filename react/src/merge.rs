/// Simplistic implementation of a sorting merge.
///
/// Merges two sorted lists into one sorted list in O(n).
pub fn merge<T: Copy + Ord>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T> {
    let mut output = Vec::with_capacity(a.len() + b.len());

    // reverse the lists to get fast popping
    a.reverse();
    b.reverse();

    while a.len() > 0 && b.len() > 0 {
        output.push(if a.last().unwrap() <= b.last().unwrap() {
            a.pop().unwrap()
        } else {
            b.pop().unwrap()
        });
    }
    // now, at least one of the inputs is completely drained.
    // we don't have to care which! extending with the rest of both
    // is guaranteed to produce correct results.
    // Don't forget to first restore original ordering.
    a.reverse();
    output.extend(a);
    b.reverse();
    output.extend(b);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_precedence() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        assert_eq!(merge(a.clone(), b.clone()), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge(b.clone(), a.clone()), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_interleaved() {
        let a = vec![3, 6, 9];
        let b = vec![2, 4, 8];

        assert_eq!(merge(a.clone(), b.clone()), vec![2, 3, 4, 6, 8, 9]);
        assert_eq!(merge(b.clone(), a.clone()), vec![2, 3, 4, 6, 8, 9]);
    }

    #[test]
    fn test_longer() {
        let a = vec![1, 3, 5, 7, 9];
        let b = vec![6];

        assert_eq!(merge(a.clone(), b.clone()), vec![1, 3, 5, 6, 7, 9]);
        assert_eq!(merge(b.clone(), a.clone()), vec![1, 3, 5, 6, 7, 9]);
    }

    #[test]
    fn test_duplication() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 4];

        assert_eq!(merge(a.clone(), b.clone()), vec![1, 2, 2, 3, 3, 4]);
        assert_eq!(merge(b.clone(), a.clone()), vec![1, 2, 2, 3, 3, 4]);
    }
}
