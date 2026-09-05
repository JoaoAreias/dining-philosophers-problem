mod shared;

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
