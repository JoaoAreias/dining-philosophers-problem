# Dining Philosophers Simulator

Deterministic tick-based dining-philosophers simulation, visualized on GitHub Pages via WASM. Vocabulary in CONTEXT.md; key decisions in docs/adr/.

## Hard rule: the learning-exercise boundary (docs/adr/0002)

The `engine/` crate (engine, scheduler, all strategies) is implemented **by the human only**. Never write, complete, or "fix" logic in `engine/` — the `todo!()` bodies are deliberate. Allowed there: design discussion, code review, hints. Assistant-owned areas: `wasm/` glue, `web/` page, `.github/` CI, docs.

## Layout

- `engine/` — pure Rust lib (no wasm deps, natively testable). **Human-owned.**
- `wasm/` — wasm-bindgen wrapper over the engine contract (`Config`, `Simulation::{new,tick,snapshot,ticks}`, `Snapshot`). Keep 1:1, no logic.
- `web/` — static vanilla JS + SVG page, no bundler. Loads `web/pkg/` built by `wasm-pack build wasm --target web --out-dir ../web/pkg` (gitignored). Design system: docs/design-system.md — follow it for any UI change (light-only, warm paper + Fraunces/Source Serif 4/Geist Mono, philosopher faces as signature). `index.html?mock` previews the design with a scripted stand-in (no real coordination logic).
- Deploy: GitHub Actions → Pages, nothing built is committed.
