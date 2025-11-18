#!/bin/bash
export RUSTFLAGS=""
echo "Building"
RUSTFLAGS='-C target-feature=+bulk-memory' cargo build -Z build-std=std -Zunstable-options --target wasm32-unknown-unknown --release --lib
echo "Binding"
wasm-bindgen ~/.target/wasm32-unknown-unknown/release/masm.wasm --out-dir pkg/ --no-typescript --target web 
echo "Opting"
wasm-opt --all-features -O4 pkg/masm_bg.wasm -o tmp.wasm
echo "Compressing"
gzip -9 <tmp.wasm >out/masm.wasm
cat bindings.js >> pkg/masm.js
uglifyjs pkg/masm.js >out/masm.js
rm tmp.wasm
rm -r pkg