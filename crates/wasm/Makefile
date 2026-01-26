
.PHONY: deploy clean all wasm

all: wasm

wasm:
	SYSROOT="$$(rustc --print sysroot)"; \
	CARGO_HOME="$${CARGO_HOME:-$$HOME/.cargo}"; \
	RUSTFLAGS="--remap-path-prefix=$$HOME=~ --remap-path-prefix=$$CARGO_HOME=/cargo --remap-path-prefix=$$SYSROOT=/rust/sysroot" \
	wasm-pack build --no-typescript --no-opt --target no-modules --release --out-dir ./web/wasm --out-name eta --no-pack

deploy: wasm
	${MAKE} -C ./web deploy

clean:
	rm -rf ./web/wasm
	${MAKE} -C ./web clean

# alias
d: deploy
c: clean
