
.PHONY: deploy clean all wasm-pkg

all: wasm-pkg

wasm-pkg:
	SYSROOT="$$(rustc --print sysroot)"; \
	CARGO_HOME="$${CARGO_HOME:-$$HOME/.cargo}"; \
	RUSTFLAGS="--remap-path-prefix=$$HOME=~ --remap-path-prefix=$$CARGO_HOME=/cargo --remap-path-prefix=$$SYSROOT=/rust/sysroot" \
	wasm-pack build --no-typescript --no-opt --target no-modules --release --out-dir ./wasm-pkg --out-name eta --no-pack

deploy-web: wasm-pkg ./web
	${MAKE} -C ./web deploy

deploy-mail: wasm-pkg ./mail
	${MAKE} -C ./mail deploy

clean:
	rm -rf ./wasm-pkg
	${MAKE} -C ./web clean

# alias
dw: deploy-web
dm: deploy-mail
c: clean
