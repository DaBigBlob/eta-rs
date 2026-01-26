
.PHONY: deploy clean all

all: www-pop

opt-index.html: ./www/index.html
	minhtml --minify-css -o $<.opt $<
	mv $<.opt $<

opt-wasm: ./www/eta_bg.wasm
	wasm-opt -O --strip-dwarf --strip-producers --dce --coalesce-locals --enable-bulk-memory-opt -o $<.opt $<
	mv $<.opt $<

www-pop: ../wasm-pkg ./static ./3rd-party
	${MAKE} clean
	mkdir -p ./www
	for d in $^; do cp -r $$d/* ./www/; done
	find ./www/ -name .gitignore -type f -delete
	find ./www/ -name LICENSE -type f -delete

deploy: wrangler.toml www-pop
	${MAKE} opt-index.html
	${MAKE} opt-wasm
	wrangler deploy

clean:
	rm -rf ./www || true

# alias
d: deploy
c: clean
