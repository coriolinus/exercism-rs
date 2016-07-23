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
    name: String,
}

impl Robot {
    fn generate_name() -> String {
        let az_range = 'A' as u8 .. 'Z' as u8;
        let zeronine_range = '0' as u8 .. '9' as u8;
        let mut rng = thread_rng();
        sample(&mut rng, az_range, 2).iter()
            .chain(&sample(&mut rng, zeronine_range, 3))
            .map(|i| *i as char)
            .collect()
    }

    pub fn new() -> Robot {
        Robot {name: Self::generate_name()}
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generate_name()
    }
}
