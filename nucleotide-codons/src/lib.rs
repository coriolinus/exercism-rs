use std::collections::HashMap;

pub struct CodonNamer {

}

impl CodonNamer {
    pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
        if codon.len() != 3 {
            return Err("Wrong number of nucleotides in codon");
        }
        unimplemented!()
    }
}

pub fn parse(names: Vec<(&str, &str)>) -> CodonNamer {
    unimplemented!()
}
