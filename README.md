Steps to reproduce Nodejs error:

```
cargo build
wasm-bindgen --reference-types --out-dir ./dist --target nodejs ./target/wasm32-unknown-unknown/debug/wasm_node_externref.wasm
node --experimental-wasm-anyref .
```

Results in error message:

```
❯ node --experimental-wasm-anyref .
/home/marior/projects/wasm-node-externref/dist/wasm_node_externref.js:145
const wasmModule = new WebAssembly.Module(bytes);
                   ^

CompileError: WebAssembly.Module(): Compiling function #17:"wasm_bindgen::externref::Slab::alloc::ha6442f5e..." failed: i32.rem_s[1] expected type i32, found ref.null of type nullref @+7561
    at Object.<anonymous> (/home/marior/projects/wasm-node-externref/dist/wasm_node_externref.js:145:20)
    at Module._compile (internal/modules/cjs/loader.js:1256:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:1277:10)
    at Module.load (internal/modules/cjs/loader.js:1105:32)
    at Function.Module._load (internal/modules/cjs/loader.js:967:14)
    at Module.require (internal/modules/cjs/loader.js:1145:19)
    at require (internal/modules/cjs/helpers.js:75:18)
    at Object.<anonymous> (/home/marior/projects/wasm-node-externref/index.js:1:14)
    at Module._compile (internal/modules/cjs/loader.js:1256:30)
    at Object.Module._extensions..js (internal/modules/cjs/loader.js:1277:10)
```

Tested with Node 14.17.0 and 12.18.3

This bug doesn't occur with Deno v1.2.2:

```
cargo build
wasm-bindgen --reference-types --out-dir ./dist --target deno ./target/wasm32-unknown-unknown/debug/wasm_node_externref.wasm
deno run -A --v8-flags=--experimental-wasm-reftypes index.ts
```
