#!/usr/bin/env bash
# Install from a local git clone (Release download, then bundled bin/, then cargo build).
# End users: use install-cli.sh — no clone needed.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${IOS_RUNNER_VERSION:-$(grep '^version = ' extension.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')}"
INSTALL_BIN="${HOME}/.ios-runner/bin/ios-runner"

if [[ "$(uname -s)" != Darwin ]]; then
  echo "iOS Runner requires macOS." >&2
  exit 1
fi

arch_suffix() {
  case "$(uname -m)" in
    arm64) echo "aarch64-apple-darwin" ;;
    x86_64) echo "x86_64-apple-darwin" ;;
    *) echo "unsupported: $(uname -m)" >&2; exit 1 ;;
  esac
}

try_release() {
  local suffix asset url tmp
  suffix="$(arch_suffix)"
  asset="ios-runner-${suffix}"
  url="https://github.com/buds520/ios-runner/releases/download/v${VERSION}/${asset}"
  tmp="$(mktemp)"
  curl -fsSL "$url" -o "$tmp" || { rm -f "$tmp"; return 1; }
  chmod +x "$tmp"
  mv "$tmp" "$INSTALL_BIN"
}

try_local_bin() {
  local p="bin/ios-runner-$(arch_suffix)"
  [[ -x "$ROOT/$p" ]] || return 1
  cp "$ROOT/$p" "$INSTALL_BIN" && chmod +x "$INSTALL_BIN"
}

try_cargo() {
  command -v cargo >/dev/null 2>&1 || return 1
  (cd crates && cargo build -q -p ios-runner-cli --release)
  cp crates/target/release/ios-runner "$INSTALL_BIN" && chmod +x "$INSTALL_BIN"
}

echo "→ iOS Runner install (v${VERSION}, from repo)"
mkdir -p "${HOME}/.ios-runner/bin"
try_release || try_local_bin || try_cargo || {
  echo "Install failed. Try: curl -fsSL .../install-cli.sh | bash" >&2
  exit 1
}

"$INSTALL_BIN" install-self
"$INSTALL_BIN" install-zed-tasks
echo "✓ Done. Open your iOS project in Zed → Cmd+Shift+R"
