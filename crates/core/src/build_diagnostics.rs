use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::locale::t;
use crate::terminal_ui::{info, warn};

pub fn persist_build_log(content: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().context("home directory")?;
    let dir = home.join(".ios-runner/logs");
    fs::create_dir_all(&dir).context("create log dir")?;
    let ts = chrono_lite_timestamp();
    let path = dir.join(format!("build-{ts}.log"));
    fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    Ok(path)
}

fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

pub fn diagnose_build_output(log: &str) -> Vec<String> {
    let lower = log.to_lowercase();
    let mut hints = Vec::new();

    if lower.contains("signing")
        || lower.contains("development team")
        || lower.contains("provisioning profile")
    {
        hints.push(
            t(
                "签名问题：在 Xcode → Signing & Capabilities 中选择 Team，或在 config.toml 设置 development_team",
                "Signing: select a Team in Xcode → Signing & Capabilities, or set development_team in config.toml",
            )
            .to_string(),
        );
    }
    if lower.contains("pod install") || lower.contains("sandbox is not in sync") {
        hints.push(
            t(
                "依赖问题：在工程目录运行 pod install",
                "Dependencies: run pod install in the project directory",
            )
            .to_string(),
        );
    }
    if lower.contains("could not resolve package") || lower.contains("swift package") {
        hints.push(
            t(
                "Swift Package 未解析：运行 ios-runner resolve-packages",
                "Swift packages unresolved: run ios-runner resolve-packages",
            )
            .to_string(),
        );
    }
    if lower.contains("unable to find a destination") || lower.contains("ineligible destination") {
        hints.push(
            t(
                "运行目标无效：运行 ios-runner switch，或在 Zed 中运行「iOS-Runner: 选择 Scheme 与设备」",
                "Invalid destination: run ios-runner switch, or use Zed task「iOS-Runner: Select Scheme & Device」",
            )
            .to_string(),
        );
    }
    if lower.contains("error:") && (lower.contains("swift") || lower.contains("expected")) {
        hints.push(
            t(
                "编译错误：查看上方日志中的 error: 行",
                "Compile error: check error: lines in the log above",
            )
            .to_string(),
        );
    }
    if lower.contains("linker command failed") || lower.contains("ld:") {
        hints.push(
            t(
                "链接错误：检查 Framework 依赖与签名设置",
                "Link error: check frameworks and signing settings",
            )
            .to_string(),
        );
    }

    hints
}

pub fn print_build_failure_diagnostics(log: &str) {
    let hints = diagnose_build_output(log);
    if hints.is_empty() {
        return;
    }
    warn(t("构建失败 — 可能原因：", "Build failed — hints:"));
    for h in hints {
        info(&format!("  • {h}"));
    }
}

pub fn append_and_persist(stderr: &str, stdout: &str) -> Result<PathBuf> {
    let mut buf = String::new();
    if !stdout.trim().is_empty() {
        let _ = writeln!(buf, "--- stdout ---\n{stdout}");
    }
    if !stderr.trim().is_empty() {
        let _ = writeln!(buf, "--- stderr ---\n{stderr}");
    }
    if buf.is_empty() {
        buf = t("(无捕获输出)", "(no captured output)").to_string();
    }
    let path = persist_build_log(&buf)?;
    info(&format!(
        "{} {}",
        t("完整日志已保存", "Full log saved to"),
        path.display()
    ));
    print_build_failure_diagnostics(&buf);
    Ok(path)
}
