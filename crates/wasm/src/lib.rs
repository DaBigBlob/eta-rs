use eta_core::basic;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eta_runner(input: &str) -> String {
    let mut out = String::new();
    basic::runner(&mut out, input);
    out
}
