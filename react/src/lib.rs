use std::collections::{HashMap, HashSet, VecDeque};

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;

pub struct Reactor<'a, T> {
    cells: Vec<Cell<'a, T>>,
    forward_references: HashMap<CellID, VecDeque<CellID>>,
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

enum Cell<'a, T> {
    Input { value: T },
    Compute {
        cache: T,
        dependencies: Vec<CellID>,
        update: Box<Fn(&[T]) -> T + 'a>,
        callbacks: HashSet<CallbackID>,
    },
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + ::std::fmt::Display> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: Vec::new(),
            forward_references: HashMap::new(),
            callbacks: Vec::new(),
        }
    }


    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        let cid = self.cells.len();
        self.cells.push(Cell::Input { value: initial });
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
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(&mut self,
                                                 dependencies: &[CellID],
                                                 compute_func: F)
                                                 -> Result<CellID, ReactorError> {

        let cid = self.cells.len();
        // compute the initial value.
        // We unfortunately can't just call update_compute_cache here,
        // because we can't actually create the cell to insert incomplete,
        // and it's too much hassle to wrap the cache in an Option<>.
        // We just end up repeating ourselves a little instead.
        let d_values = try!(dependencies.iter()
            .map(|d_id| self.value(*d_id))
            .collect::<Option<Vec<_>>>()
            .ok_or(ReactorError::DependencyDoesntExist));
        let cache = compute_func(&d_values);

        // insert the cell into the cells vec
        self.cells.push(Cell::Compute {
            cache: cache,
            dependencies: Vec::from(dependencies),
            update: Box::new(compute_func),
            callbacks: HashSet::new(),
        });

        // ensure we can trace dependencies forward
        for dependency in dependencies {
            self.forward_references.entry(*dependency).or_insert(VecDeque::new()).push_back(cid);
        }

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
        if id >= self.cells.len() {
            None
        } else {
            match self.cells[id] {
                Cell::Input { value } => Some(value),
                Cell::Compute { cache, .. } => Some(cache),
            }
        }
    }

    /// Update a particular cell by id.
    ///
    /// If the cell is valid, return a bool indicating whether the cell changed
    /// as a result of this update.
    ///
    /// Otherwise, return a ReactorError.
    fn update_compute_cache(&mut self, id: CellID) -> Result<bool, ReactorError> {
        if id >= self.cells.len() {
            Err(ReactorError::CellDoesntExist(id))
        } else {
            // borrow self immutably a few times to figure out the new cache value
            let new_cache = try!(match self.cells[id] {
                Cell::Input { .. } => Err(ReactorError::NotAComputeCell(id)),
                Cell::Compute { ref dependencies, ref update, .. } => {
                    let d_values = try!(dependencies.iter()
                        .map(|d_id| self.value(*d_id))
                        .collect::<Option<Vec<_>>>()
                        .ok_or(ReactorError::DependencyDoesntExist));

                    Ok(update(&d_values))
                }
            });

            // now borrow self once, mutably to set the new cache value
            match self.cells[id] {
                Cell::Input { .. } => Err(ReactorError::NotAComputeCell(id)),
                Cell::Compute { ref mut cache, ref callbacks, .. } => {
                    let changed = *cache != new_cache;
                    *cache = new_cache;
                    if changed {
                        for cbid in callbacks {
                            // see http://stackoverflow.com/a/39532428/504550 for
                            // a small discussion of why this syntax is necessary.
                            // In short, the FnMut binding must itself be mutable,
                            // and the compiler isn't smart enough to figure that
                            // out itself from simply `self.callbacks[*cbid](new_cache);`
                            (*&mut self.callbacks[*cbid])(new_cache);
                        }
                    }
                    Ok(changed)
                }
            }
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
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ReactorError> {
        if id >= self.cells.len() {
            Err(ReactorError::CellDoesntExist(id))
        } else {
            match self.cells[id] {
                Cell::Compute { .. } => Err(ReactorError::NotAnInputCell(id)),
                Cell::Input { .. } => {
                    self.cells[id] = Cell::Input { value: new_value };

                    // go through and update everything which depended on this cell now
                    let mut cells_to_update = Vec::new();
                    if let Some(fwd_ref) = self.forward_references.get(&id) {
                        let mut fwd_ref = fwd_ref.clone();

                        while fwd_ref.len() > 0 {
                            let updating_cell_id = fwd_ref.pop_front().unwrap();
                            // Ideally, instead of pushing to a list of cells to update here,
                            // we'd just update the compute cache. Unfortunately, *self has
                            // been borrowed mutably once already, and we can't do it again.
                            //
                            // The implication is that we can't filter, here, only those
                            // cells whose values actually changed from the computation;
                            // we have to propagate through all updated cells even if some
                            // of them don't change.
                            cells_to_update.push(updating_cell_id);
                            if let Some(n_fwd_ref) = self.forward_references
                                .get(&updating_cell_id) {
                                fwd_ref.extend(n_fwd_ref);
                            }
                        }
                    }
                    // we depend on the fact that no cell will ever depend on
                    // a cell with a higher ID in order to ensure that no callback is called
                    // more than once.
                    cells_to_update.sort();
                    cells_to_update.dedup();
                    for cell in cells_to_update {
                        try!(self.update_compute_cache(cell));
                    }

                    Ok(())
                }
            }
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

        match self.cells[id] {
            Cell::Input { .. } => Err(ReactorError::NotAComputeCell(id)),
            Cell::Compute { ref mut callbacks, .. } => {
                callbacks.insert(cbid);
                Ok(cbid)
            }
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self,
                           cell: CellID,
                           callback: CallbackID)
                           -> Result<(), ReactorError> {
        if cell >= self.cells.len() {
            return Err(ReactorError::CellDoesntExist(cell));
        }

        // we never actually remove the callback, because that would invalidate all the other
        // callback IDs. We just stop calling it.

        match self.cells[cell] {
            Cell::Input { .. } => Err(ReactorError::NotAComputeCell(cell)),
            Cell::Compute { ref mut callbacks, .. } => {
                if callbacks.remove(&callback) {
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
