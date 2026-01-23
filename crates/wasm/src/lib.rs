use eta_core::basic::runner;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn js_runner(input: &str) -> String {
    let mut out = String::new();
    runner(&mut out, input);
    out
}

#[wasm_bindgen]
pub fn greet_me(name: &str) -> String {
    format!("Hello, {}!", name)
}
