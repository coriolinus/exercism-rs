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

enum CellType<'a, T> {
    Input,
    Compute {
        dependencies: Vec<CellID>,
        update: Box<Fn(&[T]) -> T + 'a>,
        callbacks: HashSet<CallbackID>,
    },
}

struct Cell<'a, T> {
    value: T,
    c_type: CellType<'a, T>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: Vec::new(),
            forward_references: HashMap::new(),
            callbacks: Vec::new(),
        }
    }

    /// Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell {
            value: initial,
            c_type: CellType::Input,
        });
        self.cells.len() - 1
    }

    /// get the current value of each of the specified dependencies
    fn get_d_values(&self, dependencies: &[CellID]) -> Result<Vec<T>, ReactorError> {
        dependencies.iter()
            .map(|d_id| self.value(*d_id))
            .collect::<Option<Vec<_>>>()
            .ok_or(ReactorError::DependencyDoesntExist)
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

        // Collect the current values of all dependencies.
        // We'll use this in a second to initialize the cache.
        let d_values = try!(self.get_d_values(dependencies));

        // Cell id
        let cid = self.cells.len();

        // insert the cell into the cells vec
        self.cells.push(Cell {
            value: compute_func(&d_values),
            c_type: CellType::Compute {
                dependencies: Vec::from(dependencies),
                update: Box::new(compute_func),
                callbacks: HashSet::new(),
            },
        });

        // Make a separate list of forward dependencies, to simplify the process
        // of tracing updates forward from a set_value call.
        for dependency in dependencies {
            self.forward_references.entry(*dependency).or_insert(VecDeque::new()).push_back(cid);
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
            // borrow self immutably a few times here to get a new cache value
            // Ideally, we'd be able to also pull a mutable reference to the
            // cell's callbacks out of the same match; if we could, we'd avoid
            // a second match later on. However, that would mean borrowing self
            // mutably, and that can happen only once at a time. It wouldn't work
            // here, because we need at least two immutable borrows of self:
            // one to get the references to the cell's contents, and another
            // to call self.value(). We just have to match the cell type again,
            // later, to get a mutable reference to the cell's callbacks which we
            // can use to actually call them.
            let new_cache = try!(match self.cells[id].c_type {
                CellType::Input => Err(ReactorError::NotAComputeCell(id)),
                CellType::Compute { ref dependencies, ref update, .. } => {
                    Ok(update(&try!(self.get_d_values(dependencies))))
                }
            });

            let changed = self.cells[id].value != new_cache;
            self.cells[id].value = new_cache;

            if changed {
                // we have to borrow self mutably to get access to cell_callbacks
                for cbid in try!(match self.cells[id].c_type {
                        CellType::Input => Err(ReactorError::NotAComputeCell(id)),
                        CellType::Compute { ref mut callbacks, .. } => Ok(callbacks),
                    })
                    .iter() {
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
            CellType::Compute { ref mut callbacks, .. } => {
                callbacks.insert(cbid);
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
            CellType::Compute { ref mut callbacks, .. } => {
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
