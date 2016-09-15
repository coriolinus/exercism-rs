pub struct Brackets {
    balanced: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum BracketTypes {
    Parenthesis,
    Square,
    Curly,
}

impl Brackets {
    pub fn from(input: &str) -> Brackets {
        Brackets { balanced: Brackets::balanced(input) }
    }

    fn balanced(input: &str) -> bool {
        let mut bracket_stack = Vec::new();
        for ch in input.chars() {
            use BracketTypes::*;
            match ch {
                '(' => bracket_stack.push(Parenthesis),
                '[' => bracket_stack.push(Square),
                '{' => bracket_stack.push(Curly),
                ')' => {
                    if bracket_stack.pop().unwrap() != Parenthesis {
                        return false;
                    }
                }
                ']' => {
                    if bracket_stack.pop().unwrap() != Square {
                        return false;
                    }
                }
                '}' => {
                    if bracket_stack.pop().unwrap() != Curly {
                        return false;
                    }
                }
                _ => {}
            }
        }
        bracket_stack.len() == 0
    }

    pub fn are_balanced(&self) -> bool {
        self.balanced
    }
}
