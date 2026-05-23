#!/usr/bin/env bash
# Local release/CI preflight checks.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "== Rust format =="
(cd "$ROOT/crates" && cargo fmt --check)

echo "== Unit tests =="
(cd "$ROOT/crates" && cargo test --workspace)

echo "== Clippy =="
(cd "$ROOT/crates" && cargo clippy --workspace --all-targets -- -D warnings)

echo "== Extension embeds =="
"$ROOT/scripts/sync-extension-embeds.sh" --check

echo "== Shell syntax =="
for script in "$ROOT"/scripts/*.sh; do
  bash -n "$script"
done

echo "✓ Preflight passed"
