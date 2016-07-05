pub fn raindrops(input: usize) -> String {
    let mut ret = String::new();

    if input % 3 == 0 {
        ret = ret + "Pling"
    }
    if input % 5 == 0 {
        ret = ret + "Plang"
    }
    if input % 7 == 0 {
        ret = ret + "Plong"
    }
    if ret.len() == 0 {
        ret = input.to_string()
    }

    ret
}
