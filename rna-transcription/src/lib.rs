use std::iter::FromIterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}

impl Nucleotide {
    pub fn from_char(&c: &char) -> Option<Nucleotide> {
        match c {
            'A' => Some(Nucleotide::Adenine),
            'C' => Some(Nucleotide::Cytosine),
            'G' => Some(Nucleotide::Guanine),
            'T' => Some(Nucleotide::Thymine),
            'U' => Some(Nucleotide::Uracil),
            _   => None,
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Nucleotide::Adenine => 'A',
            Nucleotide::Cytosine => 'C',
            Nucleotide::Guanine => 'G',
            Nucleotide::Thymine => 'T',
            Nucleotide::Uracil => 'U',
        }
    }
}

// newtypes for disambiguation
#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid(pub Vec<Nucleotide>);
#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid(pub Vec<Nucleotide>);

// RNA impls
impl RibonucleicAcid {
    pub fn new(s: &str) -> Option<Self> {
        s.chars().map(|c| Nucleotide::from_char(&c))
            .map(|n| match n {
                // Thymine doesn't appear in RNA
                Some(Nucleotide::Thymine) => None,
                Some(n) => Some(n),
                None => None,
            }).collect()
    }
}

impl FromIterator<Nucleotide> for RibonucleicAcid {
    fn from_iter<I: IntoIterator<Item=Nucleotide>>(iter: I) -> Self {
        RibonucleicAcid(Vec::from_iter(iter))
    }
}

// DNA impls
impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> Option<Self> {
        s.chars().map(|c| Nucleotide::from_char(&c))
            .map(|n| match n {
                // Uracil doesn't appear in DNA
                Some(Nucleotide::Uracil) => None,
                Some(n) => Some(n),
                None => None,
            }).collect()
    }

    pub fn to_rna(&self) -> Option<RibonucleicAcid> {
        self.0.iter().map(|&n| match n {
            Nucleotide::Guanine => Some(Nucleotide::Cytosine),
            Nucleotide::Cytosine => Some(Nucleotide::Guanine),
            Nucleotide::Thymine => Some(Nucleotide::Adenine),
            Nucleotide::Adenine => Some(Nucleotide::Uracil),
            _ => None,
        }).collect()
    }
}

impl FromIterator<Nucleotide> for DeoxyribonucleicAcid {
    fn from_iter<I: IntoIterator<Item=Nucleotide>>(iter: I) -> Self {
        DeoxyribonucleicAcid(Vec::from_iter(iter))
    }
}
