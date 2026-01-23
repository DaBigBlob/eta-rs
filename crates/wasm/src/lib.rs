use eta_core::basic::runner;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run(input: &str) -> String {
    let mut out = String::new();
    runner(&mut out, input);
    out
}
