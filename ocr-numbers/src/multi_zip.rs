//! Zip multiple iterators into a single iterator of vector over items.
//!
//! Acts like `zip`, but accepts an arbitrary number of input iterators, so long as they share
//! an Item type, and iterates over a vector of those Items.

pub struct MultiZip<I> {
    inputs: Vec<I>,
}

impl<I> Iterator for MultiZip<I>
    where I: Iterator
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inputs.iter_mut().map(|i| i.next()).collect::<Option<Vec<_>>>()
    }
}

/// Zip multiple iterators into a single iterator of Vec<Item>.
pub fn mzip<I>(inputs: Vec<I>) -> MultiZip<I>
    where I: Iterator
{
    MultiZip { inputs: inputs }
}
