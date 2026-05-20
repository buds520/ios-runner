#!/usr/bin/env bash
# Regenerate src/embedded_*.json for the Zed extension WASM bundle.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CHECK=0
if [[ "${1:-}" == "--check" ]]; then
  CHECK=1
elif [[ $# -gt 0 ]]; then
  echo "usage: $0 [--check]" >&2
  exit 2
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

cd "$ROOT/crates"
cargo run -q -p ios-runner-cli -- emit-global-tasks-json > "$TMP_DIR/embedded_global_tasks.json"
cargo run -q -p ios-runner-cli -- emit-embedded-keymap-json > "$TMP_DIR/embedded_keymap_entry.json"

if [[ "$CHECK" == "1" ]]; then
  diff -u "$ROOT/src/embedded_global_tasks.json" "$TMP_DIR/embedded_global_tasks.json"
  diff -u "$ROOT/src/embedded_keymap_entry.json" "$TMP_DIR/embedded_keymap_entry.json"
  echo "✓ Embedded extension JSON is up to date"
  exit 0
fi

cp "$TMP_DIR/embedded_global_tasks.json" "$ROOT/src/embedded_global_tasks.json"
cp "$TMP_DIR/embedded_keymap_entry.json" "$ROOT/src/embedded_keymap_entry.json"
echo "✓ Wrote src/embedded_global_tasks.json"
echo "✓ Wrote src/embedded_keymap_entry.json"
