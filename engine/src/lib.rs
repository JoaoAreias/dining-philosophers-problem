//! Deterministic tick-based dining-philosophers simulation.
//!
//! Everything in this crate is the learning exercise: the engine, the
//! scheduler, and every strategy are implemented by hand. The only fixed
//! surface is the contract consumed by the `wasm` glue crate and the web page:
//!
//! - [`Config`] — what a run is parameterized by
//! - [`Simulation::new`] / [`Simulation::tick`] / [`Simulation::snapshot`]
//! - [`Snapshot`] and its component types (serialized as-is to the page)
//!
//! Snapshot fields can grow freely (the page ignores what it doesn't know);
//! renaming/removing fields or changing method signatures means updating
//! `wasm/` and `web/app.js` to match.
//!
//! See docs/adr/0001-deterministic-tick-simulation.md for why there are no
//! real threads here, and CONTEXT.md for vocabulary.

use serde::Serialize;

// ---------------------------------------------------------------------------
// Contract: run configuration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StrategyId {
    Naive,
    ResourceOrdering,
    Arbiter,
    Semaphore,
    Monitor,
    ChandyMisra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchedulerMode {
    Fair,
    Random,
    Adversarial,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub strategy: StrategyId,
    pub philosophers: usize,
    pub seed: u64,
    pub scheduler: SchedulerMode,
}

// ---------------------------------------------------------------------------
// Contract: snapshots (what the page renders)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PhilosopherState {
    Thinking,
    Hungry,
    Eating,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhilosopherSnapshot {
    pub state: PhilosopherState,
    /// Fork indices currently held.
    pub holding: Vec<usize>,
    pub meals: u32,
    /// Ticks spent hungry since last meal (starvation indicator).
    pub hunger_streak: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ForkSnapshot {
    /// Philosopher index holding this fork, if any.
    pub holder: Option<usize>,
    /// Chandy–Misra only; `None` for strategies without dirty/clean forks.
    pub dirty: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Snapshot {
    pub tick: u64,
    pub philosophers: Vec<PhilosopherSnapshot>,
    pub forks: Vec<ForkSnapshot>,
    pub deadlocked: bool,
}

// ---------------------------------------------------------------------------
// Contract: the simulation
// ---------------------------------------------------------------------------

pub struct Simulation {
    // Owns table state, scheduler, strategy instances, and recorded history
    // (one Snapshot per tick) for O(1) rewind.
}

impl Simulation {
    pub fn new(config: Config) -> Self {
        let _ = config;
        todo!("build initial table state and record the tick-0 snapshot")
    }

    /// Advance one tick: poll each philosopher (in scheduler order), apply
    /// actions, resolve conflicts, record the resulting snapshot.
    pub fn tick(&mut self) {
        todo!()
    }

    /// State at tick `t`. Panics (or returns None — your call, but update the
    /// glue if you change the signature) if `t` hasn't been simulated yet.
    pub fn snapshot(&self, t: u64) -> &Snapshot {
        let _ = t;
        todo!()
    }

    /// Number of ticks simulated so far (= highest valid `snapshot` index + 1).
    pub fn ticks(&self) -> u64 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Yours to design from here down — sketches only, reshape freely.
// The glue crate does not depend on anything below this line.
// ---------------------------------------------------------------------------

/// What a philosopher decides to do at a tick. Sketch — evolve as strategies
/// demand (arbiter requests, Chandy–Misra fork-request messages, block/wake
/// for the monitor solution, ...).
pub enum Action {
    // e.g. TryTakeFork(usize), ReleaseFork(usize), Wait, ...
}

/// A strategy's read-only view of the table when polled. Sketch.
pub struct TableView {}

/// One solution to the problem, as a polled state machine.
pub trait Strategy {
    fn next_action(&mut self, view: &TableView) -> Action;
}
