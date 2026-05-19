//! Shell snippet to install `ios-runner` into `~/.ios-runner/bin` without Rust/cargo.
//!
//! Uses lowercase `ir_*` shell variables so Zed does not pre-expand them as task env vars
//! (Zed substitutes `$VAR` before the shell runs; `$IOS_RUNNER` becomes empty → `curl -o` fails).

pub const INSTALL_DIR: &str = ".ios-runner/bin";
pub const INSTALL_BIN: &str = ".ios-runner/bin/ios-runner";

/// Prepended to Zed tasks: download CLI from GitHub Releases if missing.
pub fn zed_task_preamble() -> String {
    r#"
ir_bin="$HOME/.ios-runner/bin/ios-runner"
if [ ! -x "$ir_bin" ]; then
  echo "iOS-Runner: 正在下载命令行工具（无需 cargo）…"
  mkdir -p "$HOME/.ios-runner/bin"
  ir_arch="$(uname -m)"
  case "$ir_arch" in
    arm64) ir_asset="ios-runner-aarch64-apple-darwin" ;;
    x86_64) ir_asset="ios-runner-x86_64-apple-darwin" ;;
    *) echo "不支持的 Mac 架构: $ir_arch"; exit 1 ;;
  esac
  ir_url="https://github.com/buds520/ios-runner/releases/latest/download/${ir_asset}"
  if ! curl -fsSL "$ir_url" -o "$HOME/.ios-runner/bin/ios-runner"; then
    echo "下载失败: $ir_url"
    echo "可能尚未发布 GitHub Release，请在本机执行:"
    echo "  cd ios-runner/crates && cargo install --path cli --locked && ios-runner install-self"
    exit 1
  fi
  chmod +x "$ir_bin"
  echo "✓ 已安装 $ir_bin"
fi
export PATH="$HOME/.ios-runner/bin:$PATH"
"#
    .trim()
    .to_string()
}
