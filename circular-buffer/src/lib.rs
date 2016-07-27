use std::mem;

pub type CBError<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
    ZeroCapacity,
}

pub struct CircularBuffer<T> {
    items: Vec<Option<T>>,
    read_index: usize,
    write_index: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            items: none_vec(size),
            read_index: 0,
            write_index: 0,
        }
    }

    /// Remove and return the first item.
    pub fn read(&mut self) -> CBError<T> {
        try!(self.raise_on_len_0());

        // it's only when doing data-structures stuff like this that the ++ operator makes sense
        if self.items[self.read_index].is_some() {
            let index = and_increment(&mut self.read_index, self.items.len());
            Ok(mem::replace(&mut self.items[index], None).unwrap())
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    /// Insert an item, raising an error if the buffer is full.
    pub fn write(&mut self, item: T) -> CBError<()> {
        try!(self.raise_on_len_0());

        if self.items[self.write_index].is_none() {
            let index = and_increment(&mut self.write_index, self.items.len());
            self.items[index] = Some(item);
            Ok(())
        } else {
            Err(Error::FullBuffer)
        }
    }

    /// Insert an item even if the buffer is full, overwriting from the beginning.
    pub fn overwrite(&mut self, item: T) -> Option<T> {
        if self.raise_on_len_0().is_err() {
            return None;
        }

        // increment the read index also if we're about to overwrite
        if self.items[self.write_index].is_some() {
            self.read_index += 1;
        }
        let index = and_increment(&mut self.write_index, self.items.len());
        mem::replace(&mut self.items[index], Some(item))
    }

    /// Remove all items from the buffer
    pub fn clear(&mut self) {
        self.items = none_vec(self.items.len());
        self.read_index = 0;
        self.write_index = 0;
    }

    /// Life is easier if we can assume that the length is not 0,
    /// so just try!() this at the beginning of our functions.
    fn raise_on_len_0(&self) -> CBError<()> {
        if self.items.len() == 0 {
            Err(Error::ZeroCapacity)
        } else {
            Ok(())
        }
    }
}

fn none_vec<T>(size: usize) -> Vec<Option<T>> {
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(None);
    }
    v
}

/// Return the value of the specified index, and then increment it.
fn and_increment(index: &mut usize, bound: usize) -> usize {
    let old_value = *index;
    *index += 1;
    if *index >= bound {
        *index = 0;
    }
    old_value
}
