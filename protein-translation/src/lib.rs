use std::collections::HashMap;

pub type ProteinMap = HashMap<&'static str, &'static str>;
pub type PResult = Result<&'static str, &'static str>;

pub struct Codonator {
    pm: ProteinMap,
}

pub fn parse(type_list: Vec<(&'static str, &'static str)>) -> Codonator {
    Codonator { pm: type_list.into_iter().collect() }
}

impl Codonator {
    pub fn name_for(&self, input: &str) -> PResult {
        self.pm.get(input).map(|s| s.to_owned()).ok_or("No such codon known")
    }
}
