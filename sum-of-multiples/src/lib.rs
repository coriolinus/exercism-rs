/// True if n is a multiple of any item in the slice
fn is_multiple(n: usize, values: &[usize]) -> bool {
    for value in values {
        if n % value == 0 {
            return true;
        }
    }
    false
}

pub fn sum_of_multiples(n: usize, values: &[usize]) -> usize {
    (1..n).filter(|v: &usize| is_multiple(*v, values)).sum()
}
