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

pub fn hint_xcbeautify() {
    if which_xcbeautify() {
        return;
    }
    warn(crate::locale::t(
        "提示: 安装 xcbeautify 可美化编译日志 → brew install xcbeautify",
        "Tip: install xcbeautify for prettier build logs → brew install xcbeautify",
    ));
}

fn which_xcbeautify() -> bool {
    std::process::Command::new("xcbeautify")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
