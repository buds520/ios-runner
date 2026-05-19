use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::locale::t;
use crate::terminal_ui::{info, warn};

#[derive(Debug, Deserialize)]
struct LockStateJson {
    result: LockStateResult,
}

#[derive(Debug, Deserialize)]
struct LockStateResult {
    #[serde(rename = "passcodeRequired")]
    passcode_required: Option<bool>,
    #[serde(rename = "unlockedSinceBoot")]
    unlocked_since_boot: Option<bool>,
}

/// Check lock state before install/launch; warns when the device cannot be used yet.
pub fn ensure_device_ready(device_id: &str) -> Result<()> {
    let Some(state) = query_lock_state(device_id)? else {
        return Ok(());
    };

    if state.passcode_required == Some(true) && state.unlocked_since_boot == Some(false) {
        device_locked_hint(device_id);
        bail!(
            "{}",
            t(
                "真机尚未解锁（开机后需要输入密码）",
                "Device is locked (enter passcode after boot)",
            )
        );
    }

    Ok(())
}

fn query_lock_state(device_id: &str) -> Result<Option<LockStateResult>> {
    let json_path = PathBuf::from(std::env::temp_dir()).join(format!("ios-runner-lock-{device_id}.json"));
    let _ = fs::remove_file(&json_path);

    let json_arg = json_path
        .to_str()
        .context("temp lockState json path")?
        .to_string();

    let status = Command::new("xcrun")
        .args([
            "devicectl",
            "device",
            "info",
            "lockState",
            "--device",
            device_id,
            "--quiet",
            "--json-output",
            &json_arg,
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .context("devicectl lockState")?;

    if !status.success() || !json_path.is_file() {
        return Ok(None);
    }

    let text = fs::read_to_string(&json_path).context("read lockState json")?;
    let parsed: LockStateJson = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    Ok(Some(parsed.result))
}

pub fn devicectl_failure_hint(stderr: &str, stdout: &str) -> Option<String> {
    let combined = format!("{stderr}\n{stdout}").to_lowercase();
    if combined.contains("locked")
        || combined.contains("unlock your device")
        || combined.contains("device is locked")
        || combined.contains("passcode")
        || combined.contains("screen is locked")
    {
        return Some(
            t(
                "真机处于锁屏状态：请先解锁 iPhone 并保持亮屏，必要时在手机上点击「信任此电脑」后重试。",
                "Device is locked: unlock the iPhone, keep the screen on, and trust this Mac if prompted.",
            )
            .to_string(),
        );
    }
    if combined.contains("developer mode") {
        return Some(
            t(
                "请在 iPhone 上开启「开发者模式」：设置 → 隐私与安全性 → 开发者模式。",
                "Enable Developer Mode on the iPhone: Settings → Privacy & Security → Developer Mode.",
            )
            .to_string(),
        );
    }
    if combined.contains("not trusted") || combined.contains("trust") {
        return Some(
            t(
                "请在 iPhone 上信任此 Mac：连接后解锁设备，按提示点「信任」。",
                "Trust this Mac on the iPhone: unlock the device and tap Trust when prompted.",
            )
            .to_string(),
        );
    }
    None
}

pub fn report_devicectl_failure(label: &str, stderr: &str, stdout: &str) -> Result<()> {
    if let Some(hint) = devicectl_failure_hint(stderr, stdout) {
        warn(&hint);
        bail!(
            "{}",
            crate::locale::tf(|| format!("{label} 失败"), || format!("{label} failed"))
        );
    }
    if !stderr.trim().is_empty() {
        info(stderr.trim());
    }
    bail!(
        "{}",
        crate::locale::tf(
            || format!("{label} 失败（详见上方输出）"),
            || format!("{label} failed (see output above)"),
        )
    );
}

fn device_locked_hint(device_id: &str) {
    warn(&crate::locale::tf(
        || format!(
            "真机 ({device_id}) 需要先解锁：按电源键亮屏并输入密码，保持解锁后再运行 iOS-Runner: Run。"
        ),
        || format!(
            "Device ({device_id}) must be unlocked first: wake the iPhone, enter passcode, then run iOS-Runner: Run."
        ),
    ));
}
