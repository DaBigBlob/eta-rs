
.PHONY: deploy clean all

all: www-pop

opt-index.html: ./www/index.html
	minhtml --minify-css -o $<.opt $<
	mv $<.opt $<

opt-wasm: ./www/eta_bg.wasm
	wasm-opt -O --strip-dwarf --strip-producers --dce --coalesce-locals --enable-bulk-memory-opt -o $<.opt $<
	mv $<.opt $<

www-pop: ./src/index.html ./wasm/eta_bg.wasm ./wasm/eta.js
	${MAKE} clean
	mkdir -p ./www
	cp $^ ./www/
	cp -r ./static/* ./www/

deploy: wrangler.toml www-pop
	${MAKE} opt-index.html
	${MAKE} opt-wasm
	wrangler deploy

clean:
	rm -rf ./www || true

# alias
d: deploy
c: clean
