pub const CHAR_W: usize = 3;
pub const CHAR_H: usize = 4;

pub const ZERO: &'static str = " _ \n| |\n|_|\n   ";
pub const ONE: &'static str = "   \n  |\n  |\n   ";
pub const TWO: &'static str = " _ \n _|\n|_ \n   ";
pub const THREE: &'static str = " _ \n _|\n _|\n   ";
pub const FOUR: &'static str = "   \n|_|\n  |\n   ";
pub const FIVE: &'static str = " _ \n|_ \n _|\n   ";
pub const SIX: &'static str = " _ \n|_ \n|_|\n   ";
pub const SEVEN: &'static str = " _ \n  |\n  |\n   ";
pub const EIGHT: &'static str = " _ \n|_|\n|_|\n   ";
pub const NINE: &'static str = " _ \n|_|\n _|\n   ";

pub fn render_lchar(lchar: Vec<&[char]>) -> String {
    // add one extra to CHAR_W for the newlines
    let lchar: Vec<String> = lchar.iter()
        .map(|chars| chars.iter().cloned().collect())
        .collect();
    lchar.join("\n")
}

pub fn match_lchar(lchar: Vec<&[char]>) -> char {
    match render_lchar(lchar).as_ref() {
        ZERO => '0',
        ONE => '1',
        TWO => '2',
        THREE => '3',
        FOUR => '4',
        FIVE => '5',
        SIX => '6',
        SEVEN => '7',
        EIGHT => '8',
        NINE => '9',
        _ => '?',
    }
}
