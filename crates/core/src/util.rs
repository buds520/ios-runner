use std::process::{Command, Stdio};
use std::sync::OnceLock;

static HAS_XCBEAUTIFY: OnceLock<bool> = OnceLock::new();

/// Cached `xcbeautify --version` probe (once per process).
pub fn has_xcbeautify() -> bool {
    *HAS_XCBEAUTIFY.get_or_init(|| {
        Command::new("xcbeautify")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    })
}
