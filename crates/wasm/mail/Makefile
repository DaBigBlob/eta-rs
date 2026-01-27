
.PHONY: deploy clean all

all: wasm

opt-wasm: ./wasm/eta_bg.wasm
	wasm-opt -O --strip-dwarf --strip-producers --dce --coalesce-locals --enable-bulk-memory-opt -o $<.opt $<
	mv $<.opt $<

wasm: ../wasm-pkg/mail
	${MAKE} clean
	mkdir -p ./wasm
	cp -r $^/* ./wasm/

deploy: wrangler.toml wasm
	${MAKE} opt-wasm
	wrangler deploy

clean:
	rm -rf ./wasm || true

# alias
d: deploy
c: clean
