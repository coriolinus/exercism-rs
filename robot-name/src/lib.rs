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
    pub fn new() -> Robot {
        let mut r = Robot {name: String::from("")};
        r.reset_name();
        r
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        let az_range = 'A' as u8 .. 'Z' as u8;
        let zeronine_range = '0' as u8 .. '9' as u8;
        let mut rng = thread_rng();
        self.name = sample(&mut rng, az_range, 2).iter()
            .chain(&sample(&mut rng, zeronine_range, 3))
            .map(|i| *i as char)
            .collect()

    }
}
