use std::collections::VecDeque;
use std::fmt::Display;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

enum CellType<T> {
    Input(T),
    Compute {
        func: Box<Fn(&[T]) -> T>,
        args: Vec<CellID>,
        cache_value: T,
        callbacks: Vec<usize>,
    },
}

struct Cell<T> {
    ctype: CellType<T>,
    fwd_deps: Vec<usize>,
}

impl<T: Copy + PartialEq + Display> Cell<T> {
    fn new_input(value: T) -> Cell<T> {
        Cell {
            ctype: CellType::Input(value),
            fwd_deps: Vec::new(),
        }
    }
    fn new_compute<F: 'static + Fn(&[T]) -> T>(
        dependencies: &[CellID],
        compute_func: F,
        reactor: &Reactor<T>,
    ) -> Cell<T> {
        // this must be private because we assume we've already checked
        // that all dependencies exist
        let args: Vec<T> = dependencies
            .iter()
            .map(|&a| reactor.value(a).unwrap())
            .collect();
        let prev = compute_func(&args);

        Cell {
            ctype: CellType::Compute {
                func: Box::new(compute_func),
                args: dependencies.iter().cloned().collect(),
                cache_value: prev,
                callbacks: Vec::new(),
            },
            fwd_deps: Vec::new(),
        }
    }

    fn value(&self) -> T {
        match self.ctype {
            CellType::Input(val) => val,
            CellType::Compute { cache_value, .. } => cache_value,
        }
    }
}

pub struct Reactor<'a, T> {
    cells: Vec<Cell<T>>,
    callbacks: Vec<Option<Box<FnMut(T) + 'a>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + Display> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let id = InputCellID(self.cells.len());
        self.cells.push(Cell::new_input(initial));
        id
    }

    fn get_idx(&self, id: CellID) -> Result<usize, CellID> {
        let idx = match id {
            CellID::Input(InputCellID(idx)) => idx,
            CellID::Compute(ComputeCellID(idx)) => idx,
        };
        if idx >= self.cells.len() {
            return Err(id.clone());
        }
        Ok(idx)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'static + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let id = self.cells.len();
        // iter over the dependencies twice: once to ensure that they're all valid,
        // and the second time to add the forward dependencies if everything is kosher
        for &dep in dependencies.iter() {
            self.get_idx(dep)?;
        }
        for &dep in dependencies.iter() {
            let idx = self.get_idx(dep)?;
            self.cells[idx].fwd_deps.push(id);
        }
        let new_cell = Cell::new_compute(dependencies, compute_func, self);
        self.cells.push(new_cell);
        Ok(ComputeCellID(id))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        let idx = self.get_idx(id).ok()?;
        Some(self.cells[idx].value())
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if let Ok(idx) = self.get_idx(CellID::Input(id)) {
            // println!("set_value: Found input cell at idx {}", idx);
            let mut changed;
            if let Cell {
                ctype: CellType::Input(ref mut val),
                ..
            } = self.cells[idx]
            {
                changed = *val != new_value;
                // println!(
                //     "set_value: {} -> {} (changed: {})",
                //     *val, new_value, changed
                // );
                *val = new_value;
            } else {
                unreachable!()
            }
            if changed {
                self.update_fwd(idx);
            }
            true
        } else {
            // println!("set_value: No cell found at {:?}", id);
            false
        }
    }

    fn update_fwd(&mut self, idx: usize) {
        let mut callback_cells = Vec::new();
        let mut dirty_cells = VecDeque::new();

        dirty_cells.push_back(idx);

        // println!("update_fwd: init dirty_cells: {:?}", dirty_cells);

        while let Some(idx) = dirty_cells.pop_front() {
            // println!("update_fwd: examining dirty cell {}", idx);
            dirty_cells.extend(self.cells[idx].fwd_deps.iter());
            // println!("update_fwd: new dirty cells: {:?}", dirty_cells);
            let (prev_value, new_value) = if let CellType::Compute {
                ref func,
                ref args,
                ref cache_value,
                ..
            } = self.cells[idx].ctype
            {
                let args: Vec<T> = args.iter().map(|&a| self.value(a).unwrap()).collect();
                (Some(*cache_value), Some(func(&args)))
            } else {
                (None, None)
            };
            match (prev_value, new_value) {
                (Some(pv), Some(nv)) if pv != nv => {
                    // println!("update_fwd: change compute[{}] from {} -> {}", idx, pv, nv);
                    if let CellType::Compute {
                        ref mut cache_value,
                        ..
                    } = self.cells[idx].ctype
                    {
                        *cache_value = nv;
                        callback_cells.push(idx);
                    }
                }
                (_, _) => {}
            }
        }

        // now ensure all relevant callbacks are called exactly once
        callback_cells.sort();
        callback_cells.dedup();
        for idx in callback_cells {
            if let CellType::Compute {
                ref cache_value,
                ref callbacks,
                ..
            } = self.cells[idx].ctype
            {
                for &callback_id in callbacks {
                    // println!(
                    //     "update_fwd: call callbacks[{}]({})",
                    //     callback_id, *cache_value
                    // );
                    if let Some(ref mut callback) = self.callbacks[callback_id] {
                        callback(*cache_value);
                    }
                }
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        let idx = self.get_idx(CellID::Compute(id)).ok()?;

        let cb_id = self.callbacks.len();
        self.callbacks.push(Some(Box::new(callback)));
        // println!("add_callback: pushed fn to reactor.callbacks[{}]", cb_id);

        if let Cell {
            ctype: CellType::Compute {
                ref mut callbacks, ..
            },
            ..
        } = self.cells[idx]
        {
            callbacks.push(cb_id);
        // println!("add_callback: pushed id to cell.callbacks: {:?}", callbacks);
        } else {
            unreachable!()
        }
        Some(CallbackID(cb_id))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.get_idx(CellID::Compute(cell))
            .map_err(|_| RemoveCallbackError::NonexistentCell)?;
        let CallbackID(idx) = callback;
        if idx >= self.callbacks.len() || self.callbacks[idx].is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        self.callbacks[idx] = None;
        Ok(())
    }
}
