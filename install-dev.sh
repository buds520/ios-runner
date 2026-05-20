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

resolve_src_dir() {
  if [[ -f "${SCRIPT_DIR}/extension.toml" ]]; then
    SRC_DIR="${SCRIPT_DIR}"
    echo "→ 使用当前目录的插件源码: ${SRC_DIR}"
    return
  fi

  SRC_DIR="${CACHE_DIR}"
  if [[ -d "${SRC_DIR}/.git" ]]; then
    echo "→ 更新插件源码缓存 ${SRC_DIR}"
    git -C "${SRC_DIR}" pull --ff-only
  else
    echo "→ 克隆插件源码到 ${SRC_DIR}"
    mkdir -p "$(dirname "${SRC_DIR}")"
    git clone "${REPO_URL}" "${SRC_DIR}"
  fi
}

ensure_rust
resolve_src_dir

mkdir -p "${HOME}/.ios-runner/bin"

echo "→ 编译 CLI"
(cd "${SRC_DIR}/crates" && cargo build -q -p ios-runner-cli --release)
cp "${SRC_DIR}/crates/target/release/ios-runner" "${INSTALL_BIN}"
chmod +x "${INSTALL_BIN}"

echo "→ 写入 Zed 全局任务与快捷键"
"${INSTALL_BIN}" install-zed-tasks

echo ""
echo "✓ 安装完成"
echo ""
echo "  两个目录，不要搞混："
echo "    • 插件源码（下面 ① 选这个）: ${SRC_DIR}"
echo "    • 你的 iOS App 工程（下面 ② Open Folder 选那个）"
echo ""
echo "  ① Zed → Extensions → Install Dev Extension"
echo "     选择含 extension.toml 的目录 → ${SRC_DIR}"
echo "     （不是你的 App 工程，也无需 clone 进 App 工程里）"
echo ""
echo "  ② Cmd+Q 退出 Zed 再打开 → File → Open Folder → 你的 iOS 工程"
echo "  ③ Cmd+Shift+U 初始化，Cmd+Shift+R 运行"
echo ""
echo "  检查环境（在 App 工程目录下执行）: ios-runner doctor"
