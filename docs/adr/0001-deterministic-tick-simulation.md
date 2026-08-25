# Deterministic tick simulation instead of real threads

This repo demonstrates dining-philosophers solutions on a static GitHub Pages site with VisuAlgo-style interactive playback (pause, step back, scrub, adjustable seed and scheduler). That requires the simulation to run in the browser and be fully reproducible, so the engine is a deterministic, tick-based simulation: philosophers are state machines polled once per tick, all randomness derives from an explicit seed, and every tick's snapshot is recorded for O(1) rewind.

## Considered Options

- **Real `std::thread` + `Mutex` implementations, visualized via recorded traces** — rejected: playback of canned recordings, no interactivity (no live parameter changes, no adversarial scheduling on demand).
- **Real threads in the browser via WASM** (`wasm_thread`, nightly + atomics + `coi-serviceworker` to fake COOP/COEP headers on GitHub Pages) — rejected: gnarly toolchain, nondeterministic (deadlocks can't be forced or replayed), step-back impossible.
- **Deterministic tick simulation (chosen)** — the strategies (resource ordering, arbiter, semaphore, monitors, Chandy–Misra) remain the real learning content; only the concurrency primitives are simulated. Bonus: an adversarial scheduler mode can force worst-case interleavings on demand, which real threads cannot.

## Consequences

- There is deliberately no `Mutex`/`Condvar`/`std::thread` in this repo — do not "fix" that.
- Interleaving happens only at action points within a tick; parallelism appears because activities span many ticks.
- Anything nondeterministic (time, RNG without the run seed) must stay out of the engine, or rewind breaks.
