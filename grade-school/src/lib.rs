use std::collections::BTreeMap;

pub struct School {
    students: BTreeMap<u8, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { students: BTreeMap::new() }
    }

    pub fn grades(&self) -> Vec<u8> {
        // BTreeMaps inherently sort their keys
        self.students.keys().cloned().collect()
    }

    pub fn add (&mut self, grade: u8, name: &str) {
        let mut class = self.students.entry(grade).or_insert(Vec::new());
        class.push(String::from(name));
        class.sort();
    }

    pub fn grade(&self, grade: u8) -> Option<&Vec<String>> {
        self.students.get(&grade)
    }
}
