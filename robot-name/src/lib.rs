extern crate rand;
use rand::{thread_rng, sample};

/*
These are the expected signatures:

impl Robot {
    pub fn new() -> Robot { ... }
    pub fn name<'a>(&'a self) -> &'a str { ... }
    pub fn reset_name(&mut self) { ... }
}
*/

pub struct Robot {
    name: Option<String>,
}

impl Robot {
    pub fn new() -> Robot {
        Robot {name: None}
    }

    pub fn name<'a>(&'a mut self) -> &'a str {
        if self.name.is_none() {
            self.reset_name();
        }
        &self.name.unwrap()
    }

    pub fn reset_name(&mut self) {
        let mut rng = thread_rng();
        self.name = sample(&mut rng, 'A'..'Z', 2).append(sample(&mut rng, '0'..'9', 3)).iter().collect()
    }
}
