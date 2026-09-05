/*
The scheduler has two jobs:
1. It determines which philosophers will move from thinking to hungry, and eating to thinking
2. It determines which philosopher to prioritize in case multiple of them attempt to pick up a
fork at the same time

The way it operates is:
- Fair: The philosopher's priority is rotated according to the tick of the simulation, guaranteeing
they all have their share of being first. They each spend the same time eating and thinking
- Random: Both, priority, and time spent eating and thinking is determined at random.
- Adversarial: When possible, ensures deadlock, otherwise, attempts to maximize starvation.
*/
use enum_dispatch::enum_dispatch;
use crate::state::Snapshot;
use crate::strategies::StrategyId;

pub mod random;
pub mod adversarial;
pub mod fair;

use fair::FairScheduler;
use random::RandomScheduler;
use adversarial::AdversarialScheduler;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchedulerMode {
    Fair {
        thinking_duration: usize,
        eating_duration: usize,
    },
    Random {
        seed: u64,
    },
    Adversarial,
}

#[enum_dispatch(Scheduler)]
pub enum SchedulerRuntime {
    Fair(FairScheduler),
    Random(RandomScheduler),
    Adversarial(AdversarialScheduler),
}

#[enum_dispatch]
trait Scheduler {
    fn update_philosophers_state(&mut self, strategy_id: StrategyId, snapshot: Snapshot) -> Snapshot;
    fn pool_order(&mut self, tick: usize, snapshot: &Snapshot) -> Vec<usize>;
}

impl From<SchedulerMode> for SchedulerRuntime {
    fn from(mode: SchedulerMode) -> Self {
        match mode {
            SchedulerMode::Fair {
                thinking_duration,
                eating_duration,
            } => SchedulerRuntime::Fair(FairScheduler {
                thinking_duration,
                eating_duration,
            }),
            SchedulerMode::Random { seed } => SchedulerRuntime::Random(RandomScheduler::new(seed)),
            SchedulerMode::Adversarial => SchedulerRuntime::Adversarial(AdversarialScheduler),
        }
    }
}

