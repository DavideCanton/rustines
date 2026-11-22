#!/bin/bash

set -e

# out dir is relative to the crate path
wasm-pack build --target web --no-typescript --no-pack --out-dir ../../web/assets/generated/ crates/wasm/

echo "Compressing..."
gzip -9 -f web/assets/generated/rustines_wasm_bg.wasm

# Generate version.js with the git commit hash
COMMIT_HASH=$(git rev-parse HEAD)
cat > web/assets/generated/version.js <<EOF
export const VERSION = "${COMMIT_HASH}";
EOF

echo "Generated version.js with commit ${COMMIT_HASH}"
