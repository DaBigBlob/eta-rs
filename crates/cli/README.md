# CLI for the Eta calculus

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
