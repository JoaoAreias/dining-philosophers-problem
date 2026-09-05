use crate::schedulers::SchedulerMode;
use crate::strategies::StrategyId;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub strategy: StrategyId,
    pub philosophers: usize,
    pub seed: u64,
    pub scheduler: SchedulerMode,
}
