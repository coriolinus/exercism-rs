use std::collections::HashMap;
use std::sync::{Arc, Mutex};

extern crate crossbeam;

pub fn frequency(items: &[&str], thread_count: usize) -> HashMap<char, usize> {
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let item_index = Arc::new(Mutex::new(0));

    // Launch as many threads as the function call specifies
    let mut threads = Vec::new();

    // Use scoped spawns because we know we're going to be reading from `items`
    // here in a thread-safe way and joining before the function ends, but that's
    // not apparent to the rust compiler on its own.
    crossbeam::scope(|scope| {
        for _ in 0..thread_count {
            // Shadow the variables here with clone: this doesn't actually clone
            // the data, but instead creates a new atomically-reference-counted
            // instance of the relevant variable.
            let counts = counts.clone();
            let item_index = item_index.clone();

            threads.push(scope.spawn(move || {
                // TODO: get a lock, count the letters of a line
                // get a line and ensure no other thread can get it
                let my_item_index = {
                    let mut item_index = item_index.lock().unwrap();
                    let out = item_index.clone();
                    *item_index += 1;
                    out
                };
                if my_item_index < items.len() {
                    let line = items[my_item_index];
                    // for each char, lock the shared map and increment the count
                    for ch in line.chars() {
                        *counts.lock().unwrap().entry(ch).or_insert(0) += 1;
                    }
                }

            }));
        }
    });

    // Collect thread results
    threads.into_iter()
        .map(|child| child.join())
        .collect::<Vec<_>>();

    // Extract the actual data from its thread-safety wrappers
    Arc::try_unwrap(counts).unwrap().into_inner().unwrap()
}
