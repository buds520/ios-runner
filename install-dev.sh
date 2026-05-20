#!/usr/bin/env bash
# One-shot install for Zed Dev Extension users (CLI + global Zed tasks).
set -euo pipefail

REPO_URL="${IOS_RUNNER_REPO:-https://github.com/buds520/ios-runner.git}"
SRC_DIR="${HOME}/.ios-runner/src/ios-runner"
INSTALL_BIN="${HOME}/.ios-runner/bin/ios-runner"
RUSTUP_URL="https://sh.rustup.rs"
WASM_TARGET="wasm32-wasip2"

if [[ "$(uname -s)" != Darwin ]]; then
  echo "iOS Runner requires macOS." >&2
  exit 1
fi

if ! command -v git >/dev/null 2>&1; then
  echo "Missing dependency: git (install Xcode Command Line Tools: xcode-select --install)" >&2
  exit 1
fi

source_cargo_env() {
  if [[ -f "${HOME}/.cargo/env" ]]; then
    # shellcheck disable=SC1091
    source "${HOME}/.cargo/env"
  fi
}

ensure_rust() {
  source_cargo_env

  if command -v rustup >/dev/null 2>&1; then
    echo "→ Rust (rustup) OK"
  elif command -v cargo >/dev/null 2>&1; then
    echo "Found cargo but not rustup (e.g. Homebrew Rust)." >&2
    echo "Zed Dev Extension requires rustup. Install:" >&2
    echo "  curl --proto '=https' --tlsv1.2 -sSf ${RUSTUP_URL} | sh" >&2
    exit 1
  elif [[ "${IOS_RUNNER_SKIP_RUST_INSTALL:-0}" == "1" ]]; then
    echo "Missing cargo. Unset IOS_RUNNER_SKIP_RUST_INSTALL or install rustup." >&2
    exit 1
  else
    echo "→ Installing Rust via rustup (one-time, ~1 min)..."
    curl --proto '=https' --tlsv1.2 -sSf "${RUSTUP_URL}" | sh -s -- -y --default-toolchain stable
    source_cargo_env
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found after rustup install. Run: source \"\$HOME/.cargo/env\"" >&2
    exit 1
  fi

  if ! rustup target list --installed | grep -qx "${WASM_TARGET}"; then
    echo "→ Adding ${WASM_TARGET} (Zed Dev Extension WASM build)"
    rustup target add "${WASM_TARGET}"
  fi
}

ensure_rust

mkdir -p "${HOME}/.ios-runner/bin"

if [[ -d "$SRC_DIR/.git" ]]; then
  echo "→ Updating $SRC_DIR"
  git -C "$SRC_DIR" pull --ff-only
else
  echo "→ Cloning $REPO_URL → $SRC_DIR"
  mkdir -p "$(dirname "$SRC_DIR")"
  git clone "$REPO_URL" "$SRC_DIR"
fi

echo "→ Building CLI"
(cd "$SRC_DIR/crates" && cargo build -q -p ios-runner-cli --release)
cp "$SRC_DIR/crates/target/release/ios-runner" "$INSTALL_BIN"
chmod +x "$INSTALL_BIN"

echo "→ Installing Zed tasks and keymap"
"$INSTALL_BIN" install-zed-tasks

echo "→ Verifying"
"$INSTALL_BIN" doctor || true

echo ""
echo "✓ Dev install complete."
echo "  Next: Zed → Extensions → Install Dev Extension → select: $SRC_DIR"
echo "        File → Open Folder → your iOS project → Cmd+Shift+U / Cmd+Shift+R"
