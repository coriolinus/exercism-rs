use std::cmp::Ordering;

pub fn find<Ts: AsRef<[T]>, T: Ord>(input: Ts, element: T) -> Option<usize> {
    let mut idx = 0;
    let mut input = input.as_ref();
    while input.len() > 1 {
        let half = input.len() / 2;
        input = match element.cmp(&input[half]) {
            Ordering::Less => input.split_at(half).0,
            Ordering::Greater => {
                idx += half + 1;
                input.split_at(half + 1).1
            }
            Ordering::Equal => return Some(idx + half),
        }
    }
    if input.len() == 1 && input[0] == element {
        Some(idx)
    } else {
        None
    }
}
