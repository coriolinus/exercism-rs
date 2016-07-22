use std::collections::HashMap;

pub type Domino = (usize, usize);

#[inline]
fn reverse(domino: &Domino) -> Domino {
    (domino.1, domino.0)
}

type DominoCollection = HashMap<usize, Vec<Domino>>;

fn create_collection(dominos: &[Domino]) -> DominoCollection {
    dominos.iter()
        .fold(HashMap::with_capacity(dominos.len()), |mut output, domino| {
            insert(&mut output, domino);
            output
        })
}

/// Insert a domino into a collection twice: once per side.
/// Note that the domino is flipped on insertion such that the keyed side is always
/// in position 0.
fn insert(dominos: &mut DominoCollection, domino: &Domino) {
    dominos.entry(domino.0).or_insert(Vec::new()).push(domino.clone());
    dominos.entry(domino.1).or_insert(Vec::new()).push(reverse(domino));
}

/// Remove a domino from a collection.
fn remove(dominos: &mut DominoCollection, domino: &Domino) {
    fn seek_and_remove(list: &mut Vec<Domino>, domino: &Domino) {
        let mut remove_n = None;
        for (i, d) in list.iter().enumerate() {
            if d == domino {
                remove_n = Some(i);
                break;
            }
        }
        list.swap_remove(remove_n.expect("Malformed DominoCollection didn't have an expected domino in a sublist"));
    }

    let errmsg = "Malformed DominoCollection didn't contain expected value";
    {
        let first_list = dominos.get_mut(&domino.0)
            .expect(errmsg);
        seek_and_remove(first_list, domino);
    }
    if dominos.get(&domino.0).unwrap().is_empty() {
        dominos.remove(&domino.0);
    }
    {
        let second_list = dominos.get_mut(&domino.1)
            .expect(errmsg);
        seek_and_remove(second_list, &reverse(domino));
    }
    if dominos.get(&domino.1).unwrap().is_empty() {
        dominos.remove(&domino.1);
    }
}

pub fn chain(dominos: &Vec<Domino>) -> Option<Vec<Domino>> {
    // don't iterate through all dominos to find a starting one, just take the first.
    // this is an exhaustive algorithm, so any chain starting position should be as good
    // as any other.
    match dominos.len() {
        0 => Some(Vec::new()),
        1 => {
            if dominos[0].0 == dominos[0].1 {
                Some(dominos.clone())
            } else {
                None
            }
        }
        _ => {
            let first = dominos[0];
            match chain_recursive(create_collection(&dominos[1..]), first.0, first.1) {
                None => None,
                Some(mut chain) => {
                    chain.insert(0, first);
                    Some(chain)
                }
            }
        }
    }
}

/// Recursively attempt to add links to the chain
fn chain_recursive(dominos: DominoCollection, start: usize, end: usize) -> Option<Vec<Domino>> {
    if dominos.is_empty() {
        if start == end {
            Some(Vec::new())
        } else {
            None
        }
    } else {
        match dominos.get(&end) {
            None => None,
            Some(dominos_list) => {
                for d in dominos_list {
                    let mut smaller_collection = dominos.clone();
                    remove(&mut smaller_collection, &d);
                    if let Some(recursed) = chain_recursive(smaller_collection, start, d.1) {
                        return Some(
                            recursed.into_iter().fold(vec![*d], |mut o, r| {
                                o.push(r);
                                o
                            })
                        )
                    }
                }
                None
            }
        }
    }
}
