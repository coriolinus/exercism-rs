use std::collections::HashMap;

pub struct CodonNamer<'a> {
    names: HashMap<&'a str, &'a str>,
}

impl<'a> CodonNamer<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&'a str, &'a str> {
        if codon.len() != 3 {
            return Err("Wrong number of nucleotides in codon");
        }
        match self.names.get(codon) {
            Some(name) => Ok(name),
            None => Err("No result for that codon"),
        }
    }

    fn alias(&mut self, compressed: &'a str, expansion: &'a str) {
        // we want to panic here if we've mis-coded an expansion
        let name = self.names.get(expansion).unwrap().clone();
        self.names.insert(compressed, name);
    }
}

pub fn parse<'a>(names: Vec<(&'a str, &'a str)>) -> CodonNamer<'a> {
    let mut cn = CodonNamer { names: HashMap::new() };
    for (codon, name) in names {
        cn.names.insert(codon, name);
    }

    // now we have to hard-code the aliases into the table
    // it's ugly, but they have to get in there somehow, and this
    // at least results in a quick run speed
    cn.alias("GCN", "GCT");
    cn.alias("CGN", "CGT");
    cn.alias("MGR", "CGT");
    cn.alias("AAY", "AAT");
    cn.alias("GAY", "GAT");
    cn.alias("TGY", "TGT");
    cn.alias("CAR", "CAA");
    cn.alias("GAR", "GAA");
    cn.alias("GGN", "GGT");
    cn.alias("CAY", "CAT");
    cn.alias("ATH", "ATT");
    cn.alias("YTR", "TTA");
    cn.alias("CTN", "TTA");
    cn.alias("AAR", "AAA");
    cn.alias("TTY", "TTT");
    cn.alias("CCN", "CCT");
    cn.alias("TCN", "TCT");
    cn.alias("AGY", "TCT");
    cn.alias("ACN", "ACT");
    cn.alias("TAY", "TAT");
    cn.alias("GTN", "GTT");
    cn.alias("TAR", "TAA");
    cn.alias("TRA", "TAA");

    cn
}
