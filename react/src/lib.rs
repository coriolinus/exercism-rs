use std::collections::HashMap;
use std::iter::FromIterator;

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;

pub struct Reactor<T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    next_c_id: CellID,
    next_cb_id: CallbackID,
    input_cells: HashMap<CellID, T>,
    callbacks: HashMap<CallbackID, Box<Fn(&[T]) -> T>>,
    compute_cells: HashMap<CellID, (Vec<CellID>, Box<Fn(&[T]) -> T>)>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    fn next_cell_id(&mut self) -> CellID {
        let cid = self.next_c_id;
        self.next_c_id += 1;
        cid
    }

    fn next_callback_id(&mut self) -> CallbackID {
        let cid = self.next_cb_id;
        self.next_cb_id += 1;
        cid
    }

    pub fn new() -> Self {
        Reactor {
            next_c_id: 0,
            next_cb_id: 0,
            input_cells: HashMap::new(),
            callbacks: HashMap::new(),
            compute_cells: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        let cid = self.next_cell_id();
        self.input_cells.insert(cid, initial);
        cid
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'static + Fn(&[T]) -> T>(&mut self,
                                                      dependencies: &[CellID],
                                                      compute_func: F)
                                                      -> Result<CellID, ()> {

        if !dependencies.iter().all(|depend_id| {
            self.input_cells.contains_key(depend_id) || self.compute_cells.contains_key(depend_id)
        }) {
            // not all dependencies exist
            return Err(());
        }

        let cid = self.next_cell_id();
        self.compute_cells.insert(cid,
                                  (Vec::from_iter(dependencies.iter().cloned()),
                                   Box::new(compute_func)));
        Ok(cid)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        if self.input_cells.contains_key(&id) {
            // it's ok to call cloned because all T are Copy
            self.input_cells.get(&id).cloned()
        } else if self.compute_cells.contains_key(&id) {
            let &(ref dependencies, ref compute_func) = self.compute_cells.get(&id).unwrap();

            // OK, this is going to take a little explaining. Starting from the top:
            // 1. We iterate through each dependency in our dependencies list, collecting
            //    them into an Option<Vec<T>>.
            dependencies.iter()
                .map(|dependency| self.value(*dependency))
                .collect::<Option<Vec<_>>>()
                // 2. Assuming all the dependencies existed, we now have a Some(Vec<T>)
                //    in hand. We label that as `d_values`, the values of each of our
                //    dependencies.
                .map(|d_values| {
                    compute_func(&d_values)
                })
            // If at any point in the preceeding we encountered None, meaning that an
            // individual dependency returned None when we tried to get its value, or
            // a callback didn't exist, that None propagates upward here. We end up
            // not having to unwrap _anything_, which is a good property to have.


        } else {
            None
        }
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        if self.input_cells.contains_key(&id) {
            self.input_cells.insert(id, new_value);
            Ok(())
        } else {
            Err(())
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> ()>(&mut self,
                                           id: CellID,
                                           callback: F)
                                           -> Result<CallbackID, ()> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        unimplemented!()
    }
}
