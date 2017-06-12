use std::collections::HashSet;

mod merge;
use merge::merge;

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;

pub struct Reactor<'a, T: Copy + PartialEq> {
    cells: Vec<Cell<'a, T>>,
    callbacks: Vec<Box<FnMut(T) + 'a>>,
}

#[derive(Debug)]
pub enum ReactorError {
    DependencyDoesntExist,
    CellDoesntExist(CellID),
    NotAnInputCell(CellID),
    NotAComputeCell(CellID),
    CallbackNotPresent(CallbackID),
}

enum CellType<'a, T> {
    Input,
    Compute {
        update: Box<Fn(&[T]) -> T + 'a>,
        dependency_ids: Vec<CellID>,
        callback_ids: HashSet<CallbackID>,
    },
}

struct Cell<'a, T: Copy + PartialEq> {
    value: T,
    c_type: CellType<'a, T>,
    fwd_references: Vec<CellID>,
    dirty: bool,
}

impl<'a, T: Copy + PartialEq> Cell<'a, T> {
    fn create_input(initial: T) -> Cell<'a, T> {
        Cell {
            value: initial,
            c_type: CellType::Input,
            fwd_references: Vec::new(),
            dirty: false,
        }
    }

    fn create_compute<F: Fn(&[T]) -> T + 'a>(initial: T,
                                             dependency_ids: &[CellID],
                                             update: F)
                                             -> Cell<'a, T> {
        Cell {
            value: initial,
            c_type: CellType::Compute {
                update: Box::new(update),
                dependency_ids: Vec::from(dependency_ids),
                callback_ids: HashSet::new(),
            },
            fwd_references: Vec::new(),
            dirty: false,
        }
    }
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    /// get the current value of each of the specified dependencies
    fn get_d_values(&self, dependencies: &[CellID]) -> Result<Vec<T>, ReactorError> {
        dependencies.iter()
            .map(|d_id| self.value(*d_id))
            .collect::<Option<Vec<_>>>()
            .ok_or(ReactorError::DependencyDoesntExist)
    }

    /// Trace all cells which have become dirty as a result of this operation and mark them.
    fn mark(&mut self, cell: CellID) -> Result<(), ReactorError> {
        if cell < self.cells.len() {
            self.cells[cell].dirty = true;

            // we need to clone the list of forward references to ensure we don't
            // change the list as we're iterating over it; it's an issue of needing
            // to mutably borrow what would otherwise already be immutably borrowed.
            for downstream in self.cells[cell].fwd_references.clone() {
                try!(self.mark(downstream));
            }
            Ok(())
        } else {
            Err(ReactorError::CellDoesntExist(cell))
        }
    }

    /// Update all marked cells in sequence.
    ///
    /// This is a convenience function for update_
    // fn update(&mut self, cell: CellID)
    /// Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell::create_input(initial));
        self.cells.len() - 1
    }

    /// Creates a compute cell with the specified dependencies and compute function.
    /// The compute function is expected to take in its arguments in the same order as specified in
    /// `dependencies`.
    /// You do not need to reject compute functions that expect more arguments than there are
    /// dependencies (how would you check for this, anyway?).
    ///
    /// Return an Err (and you can change the error type) if any dependency doesn't exist.
    ///
    /// Notice that there is no way to *remove* a cell.
    /// This means that you may assume, without checking, that if the dependencies exist at creation
    /// time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(&mut self,
                                                 dependencies: &[CellID],
                                                 compute_func: F)
                                                 -> Result<CellID, ReactorError> {

        let cid = self.cells.len();
        let initial = compute_func(&try!(self.get_d_values(dependencies)));
        self.cells.push(Cell::create_compute(initial, dependencies, compute_func));

        // Update the forward dependencies for all depended-on cells
        for dependency in dependencies {
            self.cells[*dependency].fwd_references.push(cid);
        }

        Ok(cid)
    }

    /// Retrieves the current value of the cell, or None if the cell does not exist.
    ///
    /// You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    /// and have a `value(&self)` method on `Cell`.
    ///
    /// It turns out this introduces a significant amount of extra complexity to this exercise.
    /// We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        if id >= self.cells.len() {
            None
        } else {
            Some(self.cells[id].value)
        }
    }


    /// Sets the value of the specified input cell.
    ///
    /// Return an Err (and you can change the error type) if the cell does not exist, or the
    /// specified cell is a compute cell, since compute cells cannot have their values directly set.
    ///
    /// Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    /// a `set_value(&mut self, new_value: T)` method on `Cell`.
    ///
    /// As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ReactorError> {
        if id >= self.cells.len() {
            Err(ReactorError::CellDoesntExist(id))
        } else {
            match self.cells[id].c_type {
                CellType::Compute { .. } => Err(ReactorError::NotAnInputCell(id)),
                CellType::Input => {
                    self.cells[id].value = new_value;

                    // go through and update everything which depended on this cell now
                    self.mark(id);
                    // then update the marked ones in order
                    unimplemented!();

                    Ok(())
                }
            }
        }
    }

    /// Adds a callback to the specified compute cell.
    ///
    /// Return an Err (and you can change the error type) if the cell does not exist.
    ///
    /// Callbacks on input cells will not be tested.
    ///
    /// The semantics of callbacks (as will be tested):
    /// For a single set_value call, each compute cell's callbacks should each be called:
    /// * Zero times if the compute cell's value did not change as a result of the set_value call.
    /// * Exactly once if the compute cell's value changed as a result of the set_value call.
    ///   The value passed to the callback should be the final value of the compute cell after the
    ///   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(&mut self,
                                          id: CellID,
                                          callback: F)
                                          -> Result<CallbackID, ReactorError> {
        if id >= self.cells.len() {
            return Err(ReactorError::CellDoesntExist(id));
        }

        // create the callback and get its ID
        let cbid = self.callbacks.len();

        self.callbacks.push(Box::new(callback));

        match self.cells[id].c_type {
            CellType::Input => Err(ReactorError::NotAComputeCell(id)),
            CellType::Compute { ref mut callback_ids, .. } => {
                callback_ids.insert(cbid);
                Ok(cbid)
            }
        }
    }

    /// Removes the specified callback, using an ID returned from add_callback.
    ///
    /// Return an Err (and you can change the error type) if either the cell or callback
    /// does not exist.
    ///
    /// A removed callback should no longer be called.
    pub fn remove_callback(&mut self,
                           cell: CellID,
                           callback: CallbackID)
                           -> Result<(), ReactorError> {
        if cell >= self.cells.len() {
            return Err(ReactorError::CellDoesntExist(cell));
        }

        // we never actually remove the callback, because that would invalidate all the other
        // callback IDs. We just stop calling it.

        match self.cells[cell].c_type {
            CellType::Input => Err(ReactorError::NotAComputeCell(cell)),
            CellType::Compute { ref mut callback_ids, .. } => {
                if callback_ids.remove(&callback) {
                    // callback did exist in the callback set
                    Ok(())
                } else {
                    // callback wasn't already in the callback set
                    Err(ReactorError::CallbackNotPresent(callback))
                }
            }
        }
    }
}
