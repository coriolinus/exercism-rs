use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter().fold(BTreeMap::new(), |mut acc, (n, v)| {
        v.iter().map(|s| acc.insert(s.to_lowercase(), *n)).collect::<Vec<_>>();
        acc
    })
}
