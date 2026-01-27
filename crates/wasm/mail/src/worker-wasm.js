import * as imports from "../wasm/eta_bg.js";

import wkmod from "../wasm/eta_bg.wasm";

const instance = new WebAssembly.Instance(wkmod, {
    "../wasm/eta_bg.js": imports,
});
imports.__wbg_set_wasm(instance.exports);

export * from "../wasm/eta_bg.js";
