/// What a philosopher decides to do at a tick. Sketch — evolve as strategies
/// demand (arbiter requests, Chandy–Misra fork-request messages, block/wake
/// for the monitor solution, ...).
pub enum Action {
    // e.g. TryTakeFork(usize), ReleaseFork(usize), Wait, ...
}

/// A strategy's read-only view of the table when polled. Sketch.
pub struct TableView {}

pub trait Strategy {
    type View<'a>;
    fn next_action(&mut self, view: &Self::View<'_>) -> Action;
}
