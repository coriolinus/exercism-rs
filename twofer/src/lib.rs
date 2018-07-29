pub fn twofer(mut name: &str) -> String {
    if name == "" {
        name = "you";
    }
    format!("One for {}, one for me.", name)
}
