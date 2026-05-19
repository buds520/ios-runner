//! Shell snippet prepended to Zed tasks.
//!
//! Zed pre-expands `$VAR` before zsh runs. Use `$HOME` (env) in quoted paths, not `${HOME}`.

use crate::locale::Lang;

pub const INSTALL_DIR: &str = ".ios-runner/bin";

/// Quoted CLI path for task scripts (`$HOME` is preserved by Zed).
pub const CLI_PATH_SHELL: &str = "\"$HOME/.ios-runner/bin/ios-runner\"";

const PREAMBLE_ZH: &str = r#"
test -x "$HOME/.ios-runner/bin/ios-runner" || { command -v ios-runner >/dev/null 2>&1 && ios-runner install-self; }
test -x "$HOME/.ios-runner/bin/ios-runner" || {
  echo "iOS-Runner: 未找到命令行工具。"
  echo "请在终端执行: ios-runner install-self"
  echo "然后重新加载 Zed 窗口。"
  exit 1
}
export PATH="$HOME/.ios-runner/bin:$PATH"
"#;

const PREAMBLE_EN: &str = r#"
test -x "$HOME/.ios-runner/bin/ios-runner" || { command -v ios-runner >/dev/null 2>&1 && ios-runner install-self; }
test -x "$HOME/.ios-runner/bin/ios-runner" || {
  echo "iOS-Runner: CLI not found."
  echo "Run in a terminal: ios-runner install-self"
  echo "Then reload the Zed window."
  exit 1
}
export PATH="$HOME/.ios-runner/bin:$PATH"
"#;

/// Prepended to Zed tasks.
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
