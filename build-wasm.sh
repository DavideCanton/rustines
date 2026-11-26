#!/bin/bash

set -e

export MSYS_NO_PATHCONV=1

FOLDER=src/app/generated
BASE_HREF="${1:-/}"

# out dir is relative to the crate path
wasm-pack build --target web --no-pack --out-dir ../../web/$FOLDER/ crates/wasm/

echo "Compressing..."
gzip -9 -f web/$FOLDER/rustines_wasm_bg.wasm

echo "Moving compressed wasm to public folder"
mkdir -p web/public/generated/
mv web/$FOLDER/rustines_wasm_bg.wasm.gz web/public/generated/

# Generate version.ts with the git commit hash
COMMIT_HASH=$(git rev-parse HEAD)
cat > web/$FOLDER/version.ts <<EOF
export const VERSION = "${COMMIT_HASH}";
EOF

echo "Generated version.ts with commit ${COMMIT_HASH}"

echo "Building Angular with base-href: ${BASE_HREF}"
(cd web/ && npm run ng -- build --base-href "${BASE_HREF}")
