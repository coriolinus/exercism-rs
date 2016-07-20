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
        unimplemented!()
    }

    pub fn name<'a>(&'a self) -> &'a str {
        unimplemented!()
    }

    pub fn reset_name(&mut self) {
        
    }
}
