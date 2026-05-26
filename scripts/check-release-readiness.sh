#!/usr/bin/env bash
# Validate release-critical metadata and artifacts.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="${1:-}"
for arg in "${@:2}"; do
  case "$arg" in
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done

read_toml_version() {
  awk -F'"' '/^version = "/ { print $2; exit }' "$1"
}

EXT_VERSION="$(read_toml_version "$ROOT/extension.toml")"
ROOT_VERSION="$(read_toml_version "$ROOT/Cargo.toml")"
CRATES_VERSION="$(read_toml_version "$ROOT/crates/Cargo.toml")"

if [[ -z "$VERSION" ]]; then
  VERSION="$EXT_VERSION"
fi

failures=0

check_equal() {
  local label="$1"
  local actual="$2"
  local expected="$3"
  if [[ "$actual" == "$expected" ]]; then
    echo "✓ ${label}: ${actual}"
  else
    echo "✗ ${label}: ${actual} (expected ${expected})" >&2
    failures=$((failures + 1))
  fi
}

check_equal "extension.toml version" "$EXT_VERSION" "$VERSION"
check_equal "Cargo.toml version" "$ROOT_VERSION" "$VERSION"
check_equal "crates/Cargo.toml version" "$CRATES_VERSION" "$VERSION"

if grep -Eq "^## \\[?${VERSION}\\]?" "$ROOT/CHANGELOG.md"; then
  echo "✓ CHANGELOG.md has ${VERSION} section"
else
  echo "✗ CHANGELOG.md is missing a ${VERSION} section" >&2
  failures=$((failures + 1))
fi

if [[ "$failures" -gt 0 ]]; then
  exit 1
fi

echo "✓ Release readiness checks passed for ${VERSION}"
