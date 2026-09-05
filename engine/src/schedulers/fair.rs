use crate::state::{PhilosopherState, Snapshot};
use crate::strategies::StrategyId;
use crate::schedulers::Scheduler;

pub struct FairScheduler {
    pub thinking_duration: usize,
    pub eating_duration: usize,
}

impl Scheduler for FairScheduler {
    fn update_philosophers_state(
        &mut self,
        _strategy_id: StrategyId,
        mut snapshot: Snapshot,
    ) -> Snapshot {
        for philosopher in snapshot.philosophers.iter_mut() {
            match philosopher.state {
                PhilosopherState::Eating
                if philosopher.ticks_since_last_update >= self.eating_duration =>
                    {
                        philosopher.update_state(PhilosopherState::Thinking);
                    }
                PhilosopherState::Thinking
                if philosopher.ticks_since_last_update >= self.thinking_duration =>
                    {
                        philosopher.update_state(PhilosopherState::Hungry);
                    }
                _ => {}
            }
        }
        snapshot
    }
    fn pool_order(&mut self, tick: usize, snapshot: &Snapshot) -> Vec<usize> {
        let n_philosophers = snapshot.philosophers.len();
        (0..n_philosophers).map(|x| (x + tick) % n_philosophers).collect()
    }
}
