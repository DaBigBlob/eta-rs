
.PHONY: deploy clean all wasm

all: wasm

wasm:
	wasm-pack build --no-typescript --target web --release --out-dir ./web/wasm --out-name eta --no-pack

deploy: wasm
	${MAKE} -C ./web deploy

clean:
	rm -rf ./web/wasm
	${MAKE} -C ./web clean

# alias
d: deploy
c: clean
