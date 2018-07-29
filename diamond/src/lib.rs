use std::iter;

fn build_string(index: u8, width: u8) -> String {
    let outer = (((width - 1) / 2) - index) as usize;
    let ch = ('A' as u8 + index) as char;
    if index == 0 {
        iter::repeat(' ')
            .take(outer)
            .chain(iter::once(ch))
            .chain(iter::repeat(' ').take(outer))
            .collect()
    } else {
        let inner = (width - (2 * outer as u8) - 2) as usize;
        iter::repeat(' ')
            .take(outer)
            .chain(iter::once(ch))
            .chain(iter::repeat(' ').take(inner))
            .chain(iter::once(ch))
            .chain(iter::repeat(' ').take(outer))
            .collect()
    }
}

pub fn get_diamond(c: char) -> Vec<String> {
    if !c.is_ascii_alphabetic() {
        return Vec::new();
    }
    let c = c.to_ascii_uppercase();
    let index = (c as u8) - ('A' as u8);
    let width = 1 + (2 * index);

    let mut out = Vec::with_capacity(width as usize);

    for row in 0..=index {
        out.push(build_string(row, width))
    }
    for row in 0..index {
        out.push(build_string(index - row - 1, width))
    }

    out
}
