# Dining Philosophers Simulator

A deterministic, tick-based simulation of the dining philosophers problem, visualized interactively (VisuAlgo-style) on a static GitHub Pages site via WASM. Strategy and engine implementations are the learning exercise; visualization is supporting machinery.

## Language

**Philosopher**:
A participant at the table that alternates between thinking and eating, and must hold both adjacent forks to eat.

**Fork**:
A shared resource placed between two adjacent philosophers. Exactly one philosopher can hold it at a time.
_Avoid_: Chopstick

**Strategy**:
One solution to the problem, implemented as a polled state machine: given a view of the table, it returns the philosopher's next action. Planned strategies: naive, resource ordering, arbiter, N−1 semaphore, monitors, Chandy–Misra.
_Avoid_: Solution, algorithm (in code)

**Tick**:
The atomic unit of simulated time. Every philosopher advances once per tick; activities span many ticks, so philosophers visibly act in parallel.
_Avoid_: Step, frame

**Action**:
What a philosopher's strategy decides to do at a tick (e.g. try to take a fork, wait, release, send a request). The engine applies actions and resolves conflicts.

**Engine**:
The simulation core: owns table state, advances ticks, applies actions, resolves same-tick conflicts, and records history for playback.
_Avoid_: Simulator, runtime

**Scheduler**:
The engine component that decides the order philosophers are polled within a tick — the knob that resolves same-tick conflicts deterministically. Swappable modes: fair, random, adversarial (actively maximizes contention).

**Snapshot**:
The complete observable state of the table at one tick, recorded per tick for playback. The page renders snapshots; the engine owns their shape.

**Seed**:
The value that fully determines a run: activity durations and any scheduler randomness derive from it, making every run reproducible and rewindable.
