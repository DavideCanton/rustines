#!/bin/bash

set -e

folder=src/app/generated

# out dir is relative to the crate path
wasm-pack build --target web --no-pack --out-dir ../../web/$folder/ crates/wasm/

echo "Compressing..."
gzip -9 -f web/$folder/rustines_wasm_bg.wasm

echo "Moving compressed wasm to public folder"
mkdir -p web/public/generated/
mv web/$folder/rustines_wasm_bg.wasm.gz web/public/generated/

# Generate version.ts with the git commit hash
COMMIT_HASH=$(git rev-parse HEAD)
cat > web/$folder/version.ts <<EOF
export const VERSION = "${COMMIT_HASH}";
EOF

echo "Generated version.ts with commit ${COMMIT_HASH}"

(cd web/ && npm run ng build)
