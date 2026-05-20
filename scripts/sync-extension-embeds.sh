#!/usr/bin/env bash
# Regenerate src/embedded_*.json for the Zed extension WASM bundle.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/crates"

cargo run -q -p ios-runner-cli -- emit-global-tasks-json > "$ROOT/src/embedded_global_tasks.json"
cargo run -q -p ios-runner-cli -- emit-embedded-keymap-json > "$ROOT/src/embedded_keymap_entry.json"
echo "✓ Wrote src/embedded_global_tasks.json"
echo "✓ Wrote src/embedded_keymap_entry.json"
