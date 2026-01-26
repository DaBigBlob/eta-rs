
.PHONY: deploy clean all

all: www-pop

opt-index.html: ./www/index.html
	mv $< ./www/index.pre-opt.html
	minhtml --minify-css --minify-js ./www/index.pre-opt.html -o $<
	rm ./www/index.pre-opt.html || true

opt-wasm: ./www/eta_bg.wasm
	wasm-opt -O --strip-dwarf --strip-producers --dce --coalesce-locals --enable-bulk-memory-opt -o ./www/eta_bg.opt.wasm $^
	mv ./www/eta_bg.opt.wasm $^

www-pop: ./src/index.html ./wasm/eta_bg.wasm ./wasm/eta.js
	${MAKE} clean
	mkdir -p ./www
	cp $^ ./www/
	cp ./static/* ./www/

deploy: wrangler.toml www-pop
	${MAKE} opt-index.html
	${MAKE} opt-wasm
	wrangler deploy

clean:
	rm -rf ./www || true

# alias
d: deploy
c: clean
