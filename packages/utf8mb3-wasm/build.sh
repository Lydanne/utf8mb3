#!/bin/bash
wasm-pack build -d ./wasm-libs/utf8mb3-wasm-esm --target bundler --out-name utf8mb3-wasm-esm
wasm-pack build -d ./wasm-libs/utf8mb3-wasm-web --target web --out-name utf8mb3-wasm-web
wasm-pack build -d ./wasm-libs/utf8mb3-wasm-cjs --target nodejs --out-name utf8mb3-wasm-cjs