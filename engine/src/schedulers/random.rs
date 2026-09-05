use rand::prelude::*;
use crate::state::{PhilosopherState, Snapshot};
use crate::strategies::StrategyId;
use crate::schedulers::Scheduler;


pub struct RandomScheduler {
    pub seed: u64,
    rng: StdRng,
}

impl RandomScheduler {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Scheduler for RandomScheduler {
    fn update_philosophers_state(
        &mut self,
        _strategy_id: StrategyId,
        mut snapshot: Snapshot,
    ) -> Snapshot {
        for philosopher in snapshot.philosophers.iter_mut() {
            match philosopher.state {
                PhilosopherState::Thinking if self.rng.random_bool(0.5) => {
                    philosopher.update_state(PhilosopherState::Hungry)
                }
                PhilosopherState::Eating if self.rng.random_bool(0.5) => {
                    philosopher.update_state(PhilosopherState::Thinking)
                }
                _ => {}
            }
        }
        snapshot
    }
    fn pool_order(&mut self, _tick: usize, snapshot: &Snapshot) -> Vec<usize> {
        let n_philosophers = snapshot.philosophers.len();
        let mut indices: Vec<usize> = (0..n_philosophers).collect();
        indices.shuffle(&mut self.rng);
        indices
    }
}