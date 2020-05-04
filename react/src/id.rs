/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputCellID(pub(crate) usize);
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
pub struct ComputeCellID(pub(crate) usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallbackID(pub(crate) usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    pub(crate) fn idx(&self) -> usize {
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
