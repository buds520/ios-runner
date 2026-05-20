use std::io::{self, IsTerminal, Write};

fn use_color() -> bool {
    std::env::var("NO_COLOR").is_err()
        && std::env::var("IOS_RUNNER_NO_COLOR").is_err()
        && io::stderr().is_terminal()
}

fn style(text: &str, code: &str) -> String {
    if use_color() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

pub fn section(title: &str, detail: Option<&str>) {
    let line = "─".repeat(48);
    let _ = writeln!(io::stderr(), "{}", style(&line, "36"));
    let head = format!("  {title}");
    let _ = writeln!(io::stderr(), "{}", style(&head, "1;36"));
    if let Some(d) = detail.filter(|s| !s.is_empty()) {
        let _ = writeln!(io::stderr(), "  {d}");
    }
    let _ = writeln!(io::stderr(), "{}", style(&line, "36"));
    let _ = io::stderr().flush();
}

pub fn info(msg: &str) {
    let _ = writeln!(io::stderr(), "{}", style(msg, "90"));
}

pub fn success(msg: &str) {
    let _ = writeln!(io::stderr(), "{}", style(msg, "32"));
}

pub fn warn(msg: &str) {
    let _ = writeln!(io::stderr(), "{}", style(msg, "33"));
}

/// Warn once when user enabled xcbeautify in config but the binary is missing.
pub fn warn_xcbeautify_missing(enabled_in_config: bool) {
    if !enabled_in_config || crate::has_xcbeautify() {
        return;
    }
    warn(crate::locale::t(
        "xcbeautify 未安装，回退到原始 xcodebuild 输出。安装: brew install xcbeautify",
        "xcbeautify not installed, falling back to raw xcodebuild output. Install: brew install xcbeautify",
    ));
}
