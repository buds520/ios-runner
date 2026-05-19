//! Shell snippet to install `ios-runner` into `~/.ios-runner/bin` without Rust/cargo.

use crate::locale::Lang;

pub const INSTALL_DIR: &str = ".ios-runner/bin";

/// Pinned to the crate version baked into this binary (not GitHub `latest`).
const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

fn preamble_zh() -> String {
    format!(
        r#"
ir_bin="$HOME/.ios-runner/bin/ios-runner"
ir_version="{CLI_VERSION}"
if [ ! -x "$ir_bin" ]; then
  echo "iOS-Runner: 正在下载命令行工具（无需 cargo）…"
  mkdir -p "$HOME/.ios-runner/bin"
  ir_arch="$(uname -m)"
  case "$ir_arch" in
    arm64) ir_asset="ios-runner-aarch64-apple-darwin" ;;
    x86_64) ir_asset="ios-runner-x86_64-apple-darwin" ;;
    *) echo "不支持的 Mac 架构: $ir_arch"; exit 1 ;;
  esac
  ir_url="https://github.com/buds520/ios-runner/releases/download/v${{ir_version}}/${{ir_asset}}"
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
    )
    .trim()
    .to_string()
}

fn preamble_en() -> String {
    format!(
        r#"
ir_bin="$HOME/.ios-runner/bin/ios-runner"
ir_version="{CLI_VERSION}"
if [ ! -x "$ir_bin" ]; then
  echo "iOS-Runner: Downloading CLI (no cargo required)…"
  mkdir -p "$HOME/.ios-runner/bin"
  ir_arch="$(uname -m)"
  case "$ir_arch" in
    arm64) ir_asset="ios-runner-aarch64-apple-darwin" ;;
    x86_64) ir_asset="ios-runner-x86_64-apple-darwin" ;;
    *) echo "Unsupported Mac architecture: $ir_arch"; exit 1 ;;
  esac
  ir_url="https://github.com/buds520/ios-runner/releases/download/v${{ir_version}}/${{ir_asset}}"
  if ! curl -fsSL "$ir_url" -o "$HOME/.ios-runner/bin/ios-runner"; then
    echo "Download failed: $ir_url"
    echo "No GitHub Release yet? Run locally:"
    echo "  cd ios-runner/crates && cargo install --path cli --locked && ios-runner install-self"
    exit 1
  fi
  chmod +x "$ir_bin"
  echo "✓ Installed $ir_bin"
fi
export PATH="$HOME/.ios-runner/bin:$PATH"
"#
    )
    .trim()
    .to_string()
}

/// Prepended to Zed tasks: download CLI from GitHub Releases if missing.
pub fn zed_task_preamble(lang: Lang) -> String {
    match lang {
        Lang::ZhCn => preamble_zh(),
        Lang::En => preamble_en(),
    }
}

/// Language for task scripts: env → global config for project → global defaults → zh-CN.
pub fn lang_for_task_script(root: Option<&std::path::Path>) -> Lang {
    if let Ok(v) = std::env::var("IOS_RUNNER_LANG") {
        return Lang::parse(&v);
    }
    if let Ok(file) = crate::global_store::load_global_file() {
        if let Some(root) = root {
            let key = crate::global_store::canonical_root(root)
                .to_string_lossy()
                .to_string();
            if let Some(cfg) = file.projects.get(&key) {
                return Lang::parse(&cfg.language);
            }
        }
        return Lang::parse(&file.defaults.language);
    }
    Lang::ZhCn
}
