use crate::config::Config;
use crate::schedulers::SchedulerRuntime;
use crate::state::Snapshot;

pub struct Simulation {
    config: Config,
    scheduler: SchedulerRuntime,
    history: Vec<Snapshot>,
}

impl Simulation {
    pub fn new(config: Config) -> Self {
        let scheduler = config.scheduler.into();
        Self {
            config,
            scheduler,
            history: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        todo!()
    }

    pub fn snapshot(&self, t: u64) -> &Snapshot {
        let _ = t;
        todo!()
    }

    /// Number of ticks simulated so far (= highest valid `snapshot` index + 1).
    pub fn ticks(&self) -> usize {
        self.history.len()
    }
}
