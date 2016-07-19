use std::collections::HashMap;

type AnagramRepresentation = HashMap<char, usize>;

fn gen_ar(word: &str) -> AnagramRepresentation {
    word.chars().fold(HashMap::new(), |mut map, ch| {
        *map.entry(ch).or_insert(0) += 1;
        map
    })
}

pub fn anagrams_for(target: &str, candidates: &[&str]) -> Vec<String> {
    let target = target.to_lowercase();
    let target_ar = gen_ar(&target);
    candidates.iter()
        .filter(|&c| {
            let l = c.to_lowercase();
            l != target && gen_ar(&l) == target_ar
        })
        .map(|&s| String::from(s))
        .collect()
}
