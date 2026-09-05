use serde::Serialize;

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
    pub hunger_streak: usize,
    pub ticks_since_last_update: usize
}

impl PhilosopherSnapshot {
    pub fn update_state(&mut self, state: PhilosopherState) {
        if state == self.state { return; }

        self.state = state;
        self.ticks_since_last_update = 0;
    }
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
    pub tick: usize,
    pub philosophers: Vec<PhilosopherSnapshot>,
    pub forks: Vec<ForkSnapshot>,
    pub deadlocked: bool,
}
