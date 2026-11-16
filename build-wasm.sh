#!/bin/bash

set -e

# out dir is relative to the crate path
wasm-pack build --target web --no-typescript --no-pack --out-dir ../../web/assets/ crates/wasm/

echo "Compressing..."
gzip -9 -f web/assets/rustines_wasm_bg.wasm
