use std::collections::HashMap;

pub type Domino = (usize, usize);

#[inline]
fn reverse(domino: Domino) -> Domino {
    (domino.1, domino.0)
}

type DominoCollection = HashMap<usize, Vec<Domino>>;

fn create_collection(dominos: &[Domino]) -> DominoCollection {
    dominos.iter()
        .fold(HashMap::with_capacity(dominos.len()),
              |mut output, domino| {
                  insert(&mut output, domino);
                  output
              })
}

/// Insert a domino into a collection twice: once per side.
/// Note that the domino is flipped on insertion such that the keyed side is always
/// in position 0.
fn insert(dominos: &mut DominoCollection, domino: &Domino) {
    dominos.entry(domino.0).or_insert(Vec::new()).push(domino.clone());
    dominos.entry(domino.1).or_insert(Vec::new()).push(reverse(domino.clone()));
}

/// Remove a domino from a collection.
fn remove(dominos: &mut DominoCollection, domino: Domino) {
    fn seek_and_remove(list: &mut Vec<Domino>, domino: Domino) {
        let mut remove_n = None;
        for (i, d) in list.iter().enumerate() {
            if d == &domino {
                remove_n = Some(i);
                break;
            }
        }
        list.swap_remove(remove_n
            .expect("Malformed DominoCollection didn't have an expected domino in a sublist"));
    }

    let mut remove_by_key = |key, domino: Domino| {
        {
            let list = dominos.get_mut(&key)
                .expect("Malformed DominoCollection didn't contain expected key");
            seek_and_remove(list, domino);
        }
        if dominos.get(&key).unwrap().is_empty() {
            dominos.remove(&key);
        }
    };


    remove_by_key(domino.0, domino);
    remove_by_key(domino.1, reverse(domino));
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
            match chain_recursive(&mut create_collection(&dominos[1..]), first.0, first.1) {
                None => None,
                Some(mut chain) => {
                    chain.push(reverse(first)); // and the first shall be last...
                    Some(chain)
                }
            }
        }
    }
}

/// Recursively attempt to add links to the chain
fn chain_recursive(dominos: &mut DominoCollection,
                   start: usize,
                   end: usize)
                   -> Option<Vec<Domino>> {
    if dominos.is_empty() {
        if start == end {
            Some(Vec::new())
        } else {
            None
        }
    } else {
        for d in dominos.get(&end).unwrap_or(&Vec::new()).clone() {
            remove(dominos, d);
            if let Some(mut recursed) = chain_recursive(dominos, start, d.1) {
                return Some({
                    recursed.push(reverse(d));
                    recursed
                });
            }
            insert(dominos, &d)
        }

        None
    }
}
