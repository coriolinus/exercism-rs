pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut output = Vec::new();
    for i in min..(max + 1) {
        for j in i..(max + 1) {
            let product = i * j;
            if is_palindrome(product) {
                output.push(product);
            }
        }
    }
    output
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().map(|p| p.clone())
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().map(|p| p.clone())
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let midpoint = s.len() - (s.len() / 2);
    s.bytes()
        .take(midpoint)
        .zip(s.bytes().rev().take(midpoint))
        .all(|(l, r)| l == r)
}
