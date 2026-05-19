#!/usr/bin/env bash
# Bump semver in all package manifests. Usage: ./scripts/bump-version.sh 0.2.0
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="${1:?Usage: $0 <semver> e.g. 0.2.0}"

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
  echo "Invalid semver: $VERSION" >&2
  exit 1
fi

bump() {
  local file="$1"
  if [[ ! -f "$file" ]]; then
    echo "skip missing $file" >&2
    return
  fi
  if sed --version 2>/dev/null | grep -q GNU; then
    sed -i "s/^version = \"[0-9].*\"/version = \"${VERSION}\"/" "$file"
  else
    sed -i '' "s/^version = \"[0-9].*\"/version = \"${VERSION}\"/" "$file"
  fi
  echo "  bumped $file"
}

echo "→ version ${VERSION}"
bump "$ROOT/extension.toml"
bump "$ROOT/Cargo.toml"
bump "$ROOT/crates/Cargo.toml"
