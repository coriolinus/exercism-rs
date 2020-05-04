pub mod id;
pub use id::{CallbackID, CellID, ComputeCellID, InputCellID};

pub mod cell;
use cell::{Cell, ComputeCell, InputCell};

pub mod callback;
use callback::Callback;
pub use callback::RemoveCallbackError;

#[derive(Default)]
pub struct Reactor<'a, T> {
    cells: Vec<Cell<T>>,
    callbacks: Vec<Option<Callback<'a, T>>>,
}

impl<'a, T: Copy + PartialEq + Default> Reactor<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    fn push_cell(&mut self, cell: Cell<T>) -> usize {
        let idx = self.cells.len();
        self.cells.push(cell);
        idx
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        InputCellID(self.push_cell(Cell::Input(InputCell::new(initial))))
    }

    // Creates a compute cell with the specified dependencies and compute function.
    //
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    pub fn create_compute<F>(
        &mut self,
        dependencies: &[CellID],
        computation: F,
    ) -> Result<ComputeCellID, CellID>
    where
        F: 'static + Fn(&[T]) -> T,
    {
        let idx = ComputeCellID(self.push_cell(Cell::Compute(ComputeCell::new(
            self,
            dependencies,
            computation,
        )?)));
        for dependency in dependencies.iter() {
            if dependency.idx() >= self.cells.len() {
                return Err(*dependency);
            }
            self.cells[dependency.idx()].fwd_mut().push(idx.into());
        }
        Ok(idx)
    }

    fn idx(&self, id: CellID) -> Option<usize> {
        if id.idx() >= self.cells.len() {
            None
        } else {
            Some(id.idx())
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    pub fn value(&self, id: CellID) -> Option<T> {
        Some(self.cells[self.idx(id)?].value())
    }

    // Sets the value of the specified input cell.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let idx = match self.idx(id.into()) {
            None => return false,
            Some(idx) => idx,
        };

        let mut recompute;
        match self.cells[idx] {
            Cell::Compute(_) => return false,
            Cell::Input(ref mut ic) => {
                ic.value = new_value;

                // Construct a list of cells to recompute.
                // Rules:
                // - we can visit each cell exactly once
                // - all back-refs must be satisfied before visiting a cell
                //
                // This would be a fairly complex topo-sorting operation, but
                // we have a massive advantage: we know that all cell IDs can
                // only refer to lower cell IDs numerically. That makes things
                // simple: just traverse recursively, then sort, then dedup.
                recompute = ic.fwd.to_owned();
                // we can't use normal loop operations, because we have to
                // repeatedly extend the recompute list during iteration.
                let mut idx = 0;
                while idx < recompute.len() {
                    let ComputeCellID(id) = recompute[idx];
                    recompute.extend(self.cells[id].fwd());
                    idx += 1;
                }

                recompute.sort();
                recompute.dedup();
            }
        }

        for redo_id in recompute {
            let ComputeCellID(idx) = redo_id;

            // split the range so we can immutably borrow the lower portion while
            // we mutably borrow the upper portion
            let (lower, upper) = self.cells.split_at_mut(idx);
            match upper[0] {
                Cell::Compute(ref mut cc) => {
                    let value = cc.cache;
                    cc.recompute(lower);
                    if value != cc.cache {
                        for &CallbackID(cbid) in &cc.callbacks {
                            match self.callbacks[cbid] {
                                None => unreachable!("only legal callbacks are retained"),
                                Some(ref mut cb) => cb(cc.cache),
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        true
    }

    fn compute_mut(&mut self, id: ComputeCellID) -> Option<&mut ComputeCell<T>> {
        let ComputeCellID(idx) = id;
        if idx >= self.cells.len() {
            return None;
        }

        match self.cells[idx] {
            Cell::Input(_) => return None,
            Cell::Compute(ref mut cc) => Some(cc),
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks:
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback is the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F>(&mut self, id: ComputeCellID, callback: F) -> Option<CallbackID>
    where
        F: 'a + FnMut(T),
    {
        let cb_idx = self.callbacks.len();
        self.compute_mut(id)?.callbacks.push(CallbackID(cb_idx));
        self.callbacks.push(Some(Box::new(callback)));
        Some(CallbackID(cb_idx))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let CallbackID(idx) = callback;
        if idx >= self.callbacks.len() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        if self.callbacks[idx].is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        self.compute_mut(cell)
            .ok_or(RemoveCallbackError::NonexistentCell)?
            .callbacks
            .retain(|cbid| *cbid != callback);
        self.callbacks[idx] = None;

        Ok(())
    }
}
