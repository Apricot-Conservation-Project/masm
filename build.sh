#!/bin/bash
echo "Building"
cargo build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target wasm32-unknown-unknown --release --lib
echo "Binding"
wasm-bindgen ~/.target/wasm32-unknown-unknown/release/masm.wasm --out-dir pkg/ --no-typescript --target web
echo "Opting"
wasm-opt -O4 pkg/masm_bg.wasm -o tmp.wasm
echo "Compressing"
gzip -9 <tmp.wasm >out/masm.wasm
uglifyjs pkg/masm.js >out/masm.js
rm tmp.wasm
