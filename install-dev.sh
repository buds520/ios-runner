#!/usr/bin/env bash
# One-shot install for Zed Dev Extension users (CLI + global Zed tasks).
set -euo pipefail

REPO_URL="${IOS_RUNNER_REPO:-https://github.com/buds520/ios-runner.git}"
CACHE_DIR="${HOME}/.ios-runner/src/ios-runner"
INSTALL_BIN="${HOME}/.ios-runner/bin/ios-runner"
RUSTUP_URL="https://sh.rustup.rs"
WASM_TARGET="wasm32-wasip2"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

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
    :
  elif command -v cargo >/dev/null 2>&1; then
    echo "Found cargo but not rustup (e.g. Homebrew Rust)." >&2
    echo "Zed Dev Extension requires rustup. Install:" >&2
    echo "  curl --proto '=https' --tlsv1.2 -sSf ${RUSTUP_URL} | sh" >&2
    exit 1
  elif [[ "${IOS_RUNNER_SKIP_RUST_INSTALL:-0}" == "1" ]]; then
    echo "Missing cargo. Unset IOS_RUNNER_SKIP_RUST_INSTALL or install rustup." >&2
    exit 1
  else
    echo "→ Installing Rust (rustup)..."
    curl --proto '=https' --tlsv1.2 -sSf "${RUSTUP_URL}" | sh -s -- -y --default-toolchain stable
    source_cargo_env
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found after rustup install. Run: source \"\$HOME/.cargo/env\"" >&2
    exit 1
  fi

  if ! rustup target list --installed | grep -qx "${WASM_TARGET}"; then
    rustup target add "${WASM_TARGET}" >/dev/null
  fi
}

resolve_src_dir() {
  if [[ -f "${SCRIPT_DIR}/extension.toml" ]]; then
    SRC_DIR="${SCRIPT_DIR}"
    return
  fi

  SRC_DIR="${CACHE_DIR}"
  if [[ -d "${SRC_DIR}/.git" ]]; then
    git -C "${SRC_DIR}" pull --ff-only >/dev/null
  else
    mkdir -p "$(dirname "${SRC_DIR}")"
    git clone "${REPO_URL}" "${SRC_DIR}" >/dev/null
  fi
}

ensure_cli_on_path() {
  local bin_dir="${HOME}/.ios-runner/bin"
  local path_line="export PATH=\"${bin_dir}:\$PATH\""
  local updated=0

  for rc in "${HOME}/.zprofile" "${HOME}/.zshrc" "${HOME}/.bash_profile"; do
    [[ -f "${rc}" ]] || continue
    if grep -qF '.ios-runner/bin' "${rc}" 2>/dev/null; then
      continue
    fi
    {
      echo ""
      echo "# iOS Runner CLI"
      echo "${path_line}"
    } >>"${rc}"
    updated=1
  done

  if [[ "${updated}" -eq 0 ]] && ! grep -qF '.ios-runner/bin' "${HOME}/.zprofile" 2>/dev/null; then
    {
      echo "# iOS Runner CLI"
      echo "${path_line}"
    } >>"${HOME}/.zprofile"
    updated=1
  fi

  # shellcheck disable=SC2130
  if [[ "${updated}" -eq 1 ]]; then
    echo "→ Added ~/.ios-runner/bin to PATH (~/.zprofile or ~/.zshrc)"
  fi
  export PATH="${bin_dir}:${PATH}"
}

ensure_rust
resolve_src_dir
mkdir -p "${HOME}/.ios-runner/bin"

echo "→ Building CLI..."
(cd "${SRC_DIR}/crates" && cargo build -q -p ios-runner-cli --release)
cp "${SRC_DIR}/crates/target/release/ios-runner" "${INSTALL_BIN}"
chmod +x "${INSTALL_BIN}"
ensure_cli_on_path
"${INSTALL_BIN}" install-zed-tasks --quiet

echo ""
echo "✓ Done"
echo ""
echo "  Zed → Install Dev Extension → ${SRC_DIR}"
echo "  Cmd+Q 重启 → Open Folder → 你的 App 工程 → Cmd+Shift+U / Cmd+Shift+R"
