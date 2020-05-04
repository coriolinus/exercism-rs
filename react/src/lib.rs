/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    fn idx(&self) -> usize {
        match self {
            Self::Input(InputCellID(idx)) => *idx,
            Self::Compute(ComputeCellID(idx)) => *idx,
        }
    }
}

impl From<InputCellID> for CellID {
    fn from(id: InputCellID) -> CellID {
        Self::Input(id)
    }
}

impl From<ComputeCellID> for CellID {
    fn from(id: ComputeCellID) -> CellID {
        Self::Compute(id)
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
    fwd: Vec<ComputeCellID>,
}

impl<T> InputCell<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            fwd: Vec::new(),
        }
    }
}

struct ComputeCell<T> {
    dependencies: Vec<CellID>,
    computation: Box<dyn Fn(&[T]) -> T>,
    cache: T,
    fwd: Vec<ComputeCellID>,
}

impl<T> ComputeCell<T>
where
    T: Copy + PartialEq,
{
    fn calculate<F>(
        reactor: &Reactor<T>,
        dependencies: &[CellID],
        computation: F,
    ) -> Result<T, CellID>
    where
        F: Fn(&[T]) -> T,
    {
        let values = dependencies
            .iter()
            .map(|cid| reactor.value(*cid).ok_or(*cid))
            .collect::<Result<Vec<T>, CellID>>()?;
        Ok(computation(&values))
    }

    fn new<F>(reactor: &Reactor<T>, dependencies: &[CellID], computation: F) -> Result<Self, CellID>
    where
        F: 'static + Fn(&[T]) -> T,
    {
        Ok(Self {
            dependencies: dependencies.to_owned(),
            cache: Self::calculate(reactor, dependencies, &computation)?,
            computation: Box::new(computation),
            fwd: Vec::new(),
        })
    }

    fn recompute(&mut self, reactor: &Reactor<T>) {
        self.cache = Self::calculate(reactor, &self.dependencies, &self.computation)
            .expect("a previously valid ComputeCell was invalidated by loss of dependency");
    }
}

enum Cell<T> {
    Input(InputCell<T>),
    Compute(ComputeCell<T>),
}

impl<T> Cell<T> {
    fn fwd(&self) -> &[ComputeCellID] {
        match self {
            Self::Input(input) => &input.fwd,
            Self::Compute(compute) => &compute.fwd,
        }
    }
}

#[derive(Default)]
pub struct Reactor<T> {
    cells: Vec<Cell<T>>,
}

impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new() }
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
        Ok(ComputeCellID(self.push_cell(Cell::Compute(
            ComputeCell::new(self, dependencies, computation)?,
        ))))
    }

    // retrieve the cell at the specified index
    fn cell(&self, id: CellID) -> Option<&Cell<T>> {
        if id.idx() >= self.cells.len() {
            None
        } else {
            Some(&self.cells[id.idx()])
        }
    }

    // retrieve the cell at the specififed index mutably
    fn cell_mut(&mut self, id: CellID) -> Option<&mut Cell<T>> {
        if id.idx() >= self.cells.len() {
            None
        } else {
            Some(&mut self.cells[id.idx()])
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    pub fn value(&self, id: CellID) -> Option<T> {
        match self.cell(id)? {
            Cell::Input(ic) => Some(ic.value),
            Cell::Compute(cc) => Some(cc.cache),
        }
    }

    // Sets the value of the specified input cell.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let mut recompute;

        match self.cell_mut(id.into()) {
            None => return false,
            Some(Cell::Compute(_)) => return false,
            Some(Cell::Input(ref mut ic)) => {
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
            match self.cells[idx] {
                Cell::Compute(ref mut cc) => cc.recompute(&self),
                _ => unreachable!(),
            }
        }

        true
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
    pub fn add_callback<F: FnMut(T) -> ()>(
        &mut self,
        _id: ComputeCellID,
        _callback: F,
    ) -> Option<CallbackID> {
        unimplemented!()
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
        unimplemented!(
            "Remove the callback identified by the CallbackID {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
