//! Shell snippet prepended to Zed tasks.
//!
//! Zed pre-expands `$VAR` before zsh runs. Use `$HOME` (env) in quoted paths, not `${HOME}`.
//!
//! CLI install runs when the Zed extension loads (`Extension::new` on install / app restart).
//! Tasks assume `~/.ios-runner/bin/ios-runner` already exists.

use crate::locale::Lang;

pub const INSTALL_DIR: &str = ".ios-runner/bin";

/// Quoted CLI path for task scripts (`$HOME` is preserved by Zed).
pub const CLI_PATH_SHELL: &str = "\"$HOME/.ios-runner/bin/ios-runner\"";

const PREAMBLE_ZH: &str = r#"
export PATH="$HOME/.ios-runner/bin:$PATH"
IR="$HOME/.ios-runner/bin/ios-runner"
if ! test -x "$IR"; then
  echo ""
  echo "  iOS Runner 尚未就绪。"
  echo ""
  echo "  在 Zed 里任选一种方式（无需终端）："
  echo "    · Cmd+Q 完全退出 Zed，再重新打开"
  echo "    · 或 Cmd+Shift+P → 搜 extensions → 打开扩展页 → 重装 iOS Runner"
  echo ""
  echo "  扩展会在加载时自动安装运行环境。"
  echo ""
  exit 1
fi
"#;

const PREAMBLE_EN: &str = r#"
export PATH="$HOME/.ios-runner/bin:$PATH"
IR="$HOME/.ios-runner/bin/ios-runner"
if ! test -x "$IR"; then
  echo ""
  echo "  iOS Runner is not ready yet."
  echo ""
  echo "  In Zed (no terminal):"
  echo "    · Cmd+Q to quit Zed completely, then reopen"
  echo "    · Or Cmd+Shift+P → extensions → reinstall iOS Runner"
  echo ""
  echo "  The extension installs everything when it loads."
  echo ""
  exit 1
fi
"#;

/// Prepended to Zed tasks (PATH + extension-ready check only).
pub fn zed_task_preamble(lang: Lang) -> String {
    match lang {
        Lang::ZhCn => PREAMBLE_ZH.trim().to_string(),
        Lang::En => PREAMBLE_EN.trim().to_string(),
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
