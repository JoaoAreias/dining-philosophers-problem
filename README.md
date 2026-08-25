# Dining Philosophers Problem

Interactive, deterministic simulations of solutions to the
[dining philosophers problem](https://en.wikipedia.org/wiki/Dining_philosophers_problem),
implemented in Rust, compiled to WASM, and visualized VisuAlgo-style on GitHub Pages.

Strategies: naive (deadlocks), resource ordering, arbiter, semaphore (N−1 seats),
monitor, and Chandy–Misra — each written as a polled state machine driven by a
tick-based engine with swappable scheduler modes (fair / random / adversarial).
Every run derives from a seed, so playback supports pause, step back, and scrubbing.

Why a simulation instead of real threads: [docs/adr/0001](docs/adr/0001-deterministic-tick-simulation.md).
This is a learning exercise — the engine and strategies are human-written by design:
[docs/adr/0002](docs/adr/0002-learning-exercise-boundary.md).

## Layout

| Path | What |
|---|---|
| `engine/` | Simulation engine, scheduler, strategies (pure Rust, no wasm deps) |
| `wasm/` | Thin wasm-bindgen wrapper exposing the engine to the page |
| `web/` | Static page (vanilla JS + SVG, no bundler) |
| `docs/adr/` | Decision records · `CONTEXT.md` — vocabulary · [design system](docs/design-system.md) |

## Develop

```sh
cargo test -p engine                                       # test the engine natively
wasm-pack build wasm --target web --out-dir ../web/pkg     # build the wasm module
python3 -m http.server -d web                              # serve the page locally
```

Pushing to `main` builds and deploys the page via GitHub Actions.
