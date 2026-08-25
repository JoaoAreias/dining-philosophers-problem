# Learning-exercise boundary: engine and strategies are human-written

This project is a learning exercise. The `engine/` crate — simulation engine, scheduler (all modes), and every dining-philosophers strategy — is implemented exclusively by the project owner. AI assistance is limited to the supporting machinery: the `wasm/` glue crate, the `web/` page, CI/deploy, docs, and API co-design discussion. The type skeletons in `engine/src/lib.rs` (contract types + `todo!()` bodies) are the agreed limit of generated code in that crate.

## Consequences

- `todo!()` bodies and stub types in `engine/` are deliberate, not unfinished scaffolding — an assistant must not fill them in, "fix" them, or add strategy/engine logic, even when asked to make the build pass.
- Review, hints, and design discussion about `engine/` code are fine; authored implementation is not.
