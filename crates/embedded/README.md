# Embedded demo for Eta Core
Code under `crates/embedded` is very specific to the `esp32-s2` chip (because thats what I had laying around).\
ℹ️ **Note**: It is extremely trivial to port to other embedded devices because of the extremely freestanding nature of the Eta interpreter.
![eta_wasm_demo](../../assets/esp32s2_phy.png)
![eta_wasm_demo](../../assets/eta_esp32s2_demo.png)

### Running
```bash
cargo run --release
```
