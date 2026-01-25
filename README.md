# Platform specific Human-Interfaces for Eta-Core
Run Eta everywhere.
> This is possible because Eta Core is very thin thign and [no_std].

# CLI
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
> Note: `--target` may be specified for crosscompilation.

# Embedded devices
This is very specific to the `esp32-s2` chip.\
Why `esp32-s2`? Because that is what i had laying around.
![eta_wasm_demo](assets/esp32s2_phy.png)
![eta_wasm_demo](assets/eta_esp32s2_demo.png)

### Running
```bash
cd crates/embedded
cargo run --release
```

# Browser (WASM)
### Use at https://0xE.io
![eta_wasm_demo](assets/eta_wasm_demo.png)

### Build
```
cargo build -p eta-wasm --target wasm32-unknown-unknown --release
```

### Deploy
```bash
make -C crates/wasm deploy # this auto builds first then deploys to cf workers
```

# Linux kernel driver
> Under heavy WIP.

# Email server
> Under heavy WIP.
