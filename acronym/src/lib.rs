pub fn abbreviate(phrase: &str) -> String {
    // is this the first letter in a phrase / word?
    let mut initial = true;

    phrase.trim()
        .chars()
        .filter(|&ch| {
            if initial && ch.is_alphabetic() {
                // initial characters should be included if alphabetic
                initial = false;
                true
            } else if !ch.is_alphabetic() {
                // the next character after a non-alphabetic one should be an initial
                initial = true;
                false
            } else if ch.is_uppercase() {
                true
            } else {
                false
            }
        })
        .collect()
}
