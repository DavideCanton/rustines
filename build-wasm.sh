#!/bin/bash

wasm-pack build --profile wasm-release --target web --out-dir ../../web/assets/ crates/rustines_wasm/
