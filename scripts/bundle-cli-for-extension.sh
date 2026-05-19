#!/usr/bin/env bash
# Build (or download) macOS CLI binaries into bin/ for Zed extension packaging.
# Usage: ./scripts/bundle-cli-for-extension.sh [version]
#   version — optional; when USE_GITHUB_RELEASE=1, download tag vX.Y.Z instead of building.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${1:-$(grep '^version = ' extension.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')}"
mkdir -p bin

ARCHS=(aarch64-apple-darwin x86_64-apple-darwin)

download_release() {
  if ! command -v gh >/dev/null 2>&1; then
    echo "gh CLI required for USE_GITHUB_RELEASE=1" >&2
    exit 1
  fi
  echo "→ Downloading v${VERSION} release assets into bin/"
  gh release download "v${VERSION}" --repo buds520/ios-runner --dir bin --pattern 'ios-runner-*'
  for name in "${ARCHS[@]}"; do
    chmod +x "bin/ios-runner-${name}"
  done
}

build_local() {
  if [[ "$(uname -s)" != Darwin ]]; then
    echo "Building macOS CLI requires Darwin. Set USE_GITHUB_RELEASE=1 to download instead." >&2
    exit 1
  fi
  echo "→ Building CLI for extension bundle (v${VERSION})"
  rustup target add aarch64-apple-darwin x86_64-apple-darwin 2>/dev/null || true
  for target in "${ARCHS[@]}"; do
    (cd crates && cargo build --release -p ios-runner-cli --target "$target")
    cp "crates/target/${target}/release/ios-runner" "bin/ios-runner-${target}"
    chmod +x "bin/ios-runner-${target}"
    echo "  bin/ios-runner-${target}"
  done
}

if [[ "${USE_GITHUB_RELEASE:-0}" == "1" ]]; then
  download_release
else
  build_local
fi

echo "✓ Extension bundle ready under bin/"
