# Platform specific Human-Interfaces for Eta-Core
Run [Eta](https://github.com/DaBigBlob/eta-core) everywhere.

# CLI [native]
### Usage
```
CLI for the Eta calculus

Usage: eta [OPTIONS] [S-PAIR]

Arguments:
  [S-PAIR]  Literal S-pair input (unless --file is used)

Options:
  -f, --file <PATH>  Treat INPUT as a path to a file and read the S-pair from it
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

### Installation
```bash
cargo install eta-cli
```
> Note: Binary releases will be available later.

### Build
```bash
cargo build -p eta-cli --release
```
ℹ️ **Note**: `--target` may be specified for crosscompilation.

# Embedded devices [native]
Code under `crates/embedded` is very specific to the `esp32-s2` chip (because thats what I had laying around).\
ℹ️ **Note**: It is extremely trivial to port to other embedded devices because of the extremely freestanding nature of the Eta interpreter.
![eta_wasm_demo](assets/esp32s2_phy.png)
![eta_wasm_demo](assets/eta_esp32s2_demo.png)

### Running
```bash
cd crates/embedded
cargo run --release
```

# Browser Tab (offline) [WASM]
### Use at https://0xE.io
![eta_wasm_demo](assets/eta_browser_demo.png)
> **TODO**: Switch from `lisp-mode` to custom `eta-mode` for highlighting code.

### Build
```
cargo build -p eta-wasm --target wasm32-unknown-unknown --release
```

### Deploy
```bash
make -C crates/wasm deploy-web # this auto builds first then deploys to cf workers
```

# Email Server (CF Worker) [WASM]
### Email to [eval@0xE.io](mailto:eval@0xE.io)
![eta_wasm_demo](assets/eta_email_demo.png)
⚠️ **Note**: The reply may be placed in your junk folder.

### Build
```
cargo build -p eta-wasm --target wasm32-unknown-unknown --release
```

### Deploy
```bash
make -C crates/wasm deploy-mail # this auto builds first then deploys to cf workers
```

# Linux kernel driver
> Under heavy WIP.
