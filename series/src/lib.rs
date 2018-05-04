pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        vec![String::new(); digits.len() + 1]
    } else {
        let digits: Vec<char> = digits.chars().collect();
        digits
            .windows(len)
            .map(|chs| chs.iter().collect())
            .collect()
    }
}
