use std::collections::HashMap;

pub fn count(nucleotide: char, sequence: &str) -> usize {
    sequence.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(sequence: &str) -> HashMap<char, usize> {
    let mut ret = HashMap::new();
    for nucleotide in vec!['A', 'T', 'G', 'C'] {
        ret.insert(nucleotide, count(nucleotide, sequence));
    }
    ret
}
