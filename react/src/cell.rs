use crate::{
    id::{CallbackID, CellID, ComputeCellID},
    Reactor,
};

pub(crate) struct InputCell<T> {
    pub(crate) value: T,
    pub(crate) fwd: Vec<ComputeCellID>,
}

impl<T> InputCell<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            fwd: Vec::new(),
        }
    }
}

pub(crate) struct ComputeCell<T> {
    dependencies: Vec<CellID>,
    computation: Box<dyn Fn(&[T]) -> T>,
    pub(crate) cache: T,
    fwd: Vec<ComputeCellID>,
    pub(crate) callbacks: Vec<CallbackID>,
}

impl<T> ComputeCell<T>
where
    T: Copy + PartialEq,
{
    /// caution: use only when you know all dependencies are legal
    fn calculate<F>(cells: &[Cell<T>], dependencies: &[CellID], computation: F) -> T
    where
        F: Fn(&[T]) -> T,
    {
        let values: Vec<T> = dependencies
            .iter()
            .map(|cid| cells[cid.idx()].value())
            .collect();
        computation(&values)
    }

    pub(crate) fn new<F>(
        reactor: &Reactor<T>,
        dependencies: &[CellID],
        computation: F,
    ) -> Result<Self, CellID>
    where
        F: 'static + Fn(&[T]) -> T,
    {
        // ensure that all dependencies are legal
        if let Some(missing) = dependencies
            .iter()
            .find(|id| id.idx() >= reactor.cells.len())
        {
            return Err(*missing);
        }

        Ok(Self {
            dependencies: dependencies.to_owned(),
            cache: Self::calculate(&reactor.cells, dependencies, &computation),
            computation: Box::new(computation),
            fwd: Vec::new(),
            callbacks: Vec::new(),
        })
    }

    pub(crate) fn recompute(&mut self, cells: &[Cell<T>]) {
        self.cache = Self::calculate(cells, &self.dependencies, &self.computation);
    }
}

pub(crate) enum Cell<T> {
    Input(InputCell<T>),
    Compute(ComputeCell<T>),
}

impl<T: Copy + PartialEq> Cell<T> {
    pub(crate) fn fwd(&self) -> &[ComputeCellID] {
        match self {
            Self::Input(input) => &input.fwd,
            Self::Compute(compute) => &compute.fwd,
        }
    }

    pub(crate) fn fwd_mut(&mut self) -> &mut Vec<ComputeCellID> {
        match self {
            Self::Input(input) => &mut input.fwd,
            Self::Compute(compute) => &mut compute.fwd,
        }
    }

    pub(crate) fn value(&self) -> T {
        match self {
            Self::Input(ic) => ic.value,
            Self::Compute(cc) => cc.cache,
        }
    }
}
