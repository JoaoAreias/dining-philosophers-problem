use crate::state::{PhilosopherState, Snapshot};
use crate::strategies::StrategyId;
use crate::schedulers::Scheduler;

pub struct AdversarialScheduler;


impl Scheduler for AdversarialScheduler {

    fn update_philosophers_state(
        &mut self,
        strategy_id: StrategyId,
        mut snapshot: Snapshot,
    ) -> Snapshot {
        todo!("Implement adversarial scheduler update_philosophers_state")
    }
    fn pool_order(&mut self, tick: usize, snapshot: &Snapshot) -> Vec<usize> {
        todo!("Implement adversarial scheduler pool_order")
    }
}


// Adversarial scheduler for the naive strategy
struct AdversarialNaiveScheduler;
impl Scheduler for AdversarialNaiveScheduler {
    fn update_philosophers_state(
        &mut self,
        strategy_id: StrategyId,
        mut snapshot: Snapshot,
    ) -> Snapshot {
        // Wait until all philosophers are thinking, then turn them all to hungry at once
        let all_thinking = snapshot
            .philosophers
            .iter()
            .all(|x| x.state == PhilosopherState::Thinking);

        if all_thinking {
            for philosopher in snapshot.philosophers.iter_mut() {
                philosopher.update_state(PhilosopherState::Hungry);
            }
        } else {
            for philosopher in snapshot.philosophers.iter_mut() {
                if philosopher.state == PhilosopherState::Eating {
                    philosopher.update_state(PhilosopherState::Thinking);
                }
            }
        }
        snapshot
    }
    fn pool_order(&mut self, tick: usize, snapshot: &Snapshot) -> Vec<usize> {
        // Order doesn't really matter here
        let n_philosophers = snapshot.philosophers.len();
        (0..n_philosophers).collect()
    }
}


struct AdversarialResourceOrderingScheduler;
impl Scheduler for AdversarialResourceOrderingScheduler {
    fn update_philosophers_state(
        &mut self,
        strategy_id: StrategyId,
        mut snapshot: Snapshot,
    ) -> Snapshot {
        // Maximizes starvation for the first philosopher
        let n_philosophers = snapshot.philosophers.len();
        if snapshot.philosophers[1].state == PhilosopherState::Thinking {
            snapshot.philosophers[1].update_state(PhilosopherState::Hungry);
        }
        if snapshot.philosophers[n_philosophers - 1].state == PhilosopherState::Eating {
            snapshot.philosophers[n_philosophers - 1].update_state(PhilosopherState::Hungry)
        }
        snapshot
    }
    fn pool_order(&mut self, tick: usize, snapshot: &Snapshot) -> Vec<usize> {
        let n_philosophers = snapshot.philosophers.len();
        // First 
        (1..=n_philosophers).map(|x| x % n_philosophers).collect()
    }

}
