pub type CBError<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T> {
    items: Vec<Option<T>>,
    index: usize,
}

impl<T> CircularBuffer<T> where T: Clone {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            items: vec![None; size],
            index: 0,
        }
    }

    /// Remove and return the item at the index; increment the index
    pub fn read(&self) -> CBError<T> {
        unimplemented!()
    }

    /// Insert an item, raising an error if the buffer is full.
    /// Increment the index.
    pub fn write(&mut self, item: T) -> CBError<()> {
        unimplemented!()
    }

    /// Insert an item. If there is already an item at this index,
    /// return the former item and overwrite. Increment the index.
    pub fn overwrite(&mut self, item: T) -> Option<T> {
        unimplemented!()
    }

    /// Remove all items from the buffer
    pub fn clear(&mut self) {
        unimplemented!()
    }
}
