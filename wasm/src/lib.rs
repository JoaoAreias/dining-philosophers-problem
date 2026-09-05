//! Thin wasm-bindgen wrapper around the `engine` crate. No logic lives here:
//! it deserializes the config from JS, forwards calls, and serializes
//! snapshots back. If the engine contract changes shape, update this 1:1.

use engine::{Config, Simulation};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    // Surfaces engine panics (including `todo!()`) as readable JS errors.
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct WasmSimulation {
    inner: Simulation,
}

#[wasm_bindgen]
impl WasmSimulation {
    /// `config`: `{ strategy, philosophers, seed, scheduler }` — see
    /// `engine::Config` for the accepted values (kebab-case strings).
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Result<WasmSimulation, JsValue> {
        let config: Config =
            serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from(e.to_string()))?;
        Ok(WasmSimulation {
            inner: Simulation::new(config),
        })
    }

    pub fn tick(&mut self) {
        self.inner.tick();
    }

    pub fn snapshot(&self, t: u64) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(self.inner.snapshot(t))
            .map_err(|e| JsValue::from(e.to_string()))
    }

    pub fn ticks(&self) -> usize{
        self.inner.ticks()
    }
}
