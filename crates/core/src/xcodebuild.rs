use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::build_settings::launch_artifacts;
use crate::config::{ProjectKind, RunnerConfig};
use crate::detect::DetectedProject;
use crate::destination::{device_udid_from_destination, is_simulator_destination};
use crate::device::{ensure_device_ready, report_devicectl_failure};
use crate::simulator::{destination_for_simulator, list_simulators, udid_for_destination_name};
use crate::locale::t;
use crate::terminal_ui::{hint_xcbeautify, info, section, success, warn};

pub fn list_schemes(root: &Path, project: &DetectedProject) -> Result<Vec<String>> {
    let mut cmd = Command::new("xcodebuild");
    add_project_args(&mut cmd, project);
    cmd.arg("-list").arg("-json");

    let output = cmd
        .current_dir(root)
        .output()
        .context("run xcodebuild -list")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("xcodebuild -list failed:\n{stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: ListJson = serde_json::from_str(&stdout).context("parse xcodebuild -list JSON")?;

    let schemes = parsed
        .project
        .and_then(|p| p.schemes)
        .or(parsed.workspace.and_then(|w| w.schemes))
        .unwrap_or_default();

    Ok(schemes)
}

pub fn default_simulator_destination(
    root: &Path,
    project: &DetectedProject,
    scheme: &str,
) -> Result<String> {
    let mut cmd = Command::new("xcodebuild");
    add_project_args(&mut cmd, project);
    cmd.args(["-scheme", scheme, "-showdestinations"]);

    let output = cmd
        .current_dir(root)
        .output()
        .context("run xcodebuild -showdestinations")?;

    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.contains("platform:iOS Simulator") && trimmed.contains("name:") {
            if let Some(dest) = extract_braced_destination(trimmed) {
                return Ok(dest);
            }
        }
    }

    if let Ok(sims) = list_simulators() {
        if let Some(sim) = sims.into_iter().find(|s| s.name.starts_with("iPhone")) {
            return Ok(destination_for_simulator(&sim));
        }
    }

    bail!(
        "{}",
        t(
            "未找到可用的 iOS 模拟器，请先安装 Xcode 模拟器运行时，或运行 ios-runner configure",
            "No iOS Simulator available. Install an Xcode simulator runtime or run ios-runner configure",
        )
    )
}

fn extract_braced_destination(line: &str) -> Option<String> {
    let start = line.find('{')? + 1;
    let end = line.rfind('}')?;
    let inner = line[start..end].trim();
    if inner.is_empty() {
        return None;
    }
    Some(inner.to_string())
}

pub fn resolve_packages(root: &Path, config: &RunnerConfig) -> Result<()> {
    let mut cmd = Command::new("xcodebuild");
    add_config_args(&mut cmd, config);
    cmd.arg("-resolvePackageDependencies");

    run_command(cmd, root, "resolve package dependencies")
}

pub fn build_project(root: &Path, config: &RunnerConfig) -> Result<()> {
    section(
        t("编译", "Build"),
        Some(&format!("{} · {}", config.scheme, config.device_summary())),
    );
    hint_xcbeautify();

    if config.resolve_packages_before_build {
        info(t("解析 Swift Package…", "Resolving Swift packages…"));
        if let Err(e) = resolve_packages_quiet(root, config) {
            warn(&format!(
                "{}: {e}",
                t(
                    "Swift Package 解析失败（将继续编译）",
                    "Swift package resolve failed (continuing build)",
                )
            ));
        }
    }

    let derived = config.derived_data_path(root);
    std::fs::create_dir_all(&derived).ok();

    let mut cmd = Command::new("xcodebuild");
    add_config_args(&mut cmd, config);
    cmd.args([
        "-configuration",
        &config.configuration,
        "-destination",
        &config.destination,
        "-derivedDataPath",
        &derived.to_string_lossy(),
        "-allowProvisioningUpdates",
    ]);
    if let Some(team) = &config.development_team {
        cmd.arg(format!("DEVELOPMENT_TEAM={team}"));
    }
    cmd.arg("build");

    let raw_logs = std::env::var("IOS_RUNNER_RAW_LOG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let result = if config.xcbeautify && which_xcbeautify() && !raw_logs {
        run_xcodebuild_piped(cmd, root)
    } else {
        if raw_logs {
            info(t(
                "完整 xcodebuild 日志（IOS_RUNNER_RAW_LOG=1）",
                "Full xcodebuild log (IOS_RUNNER_RAW_LOG=1)",
            ));
        }
        run_command(cmd, root, "build")
    };
    if result.is_ok() {
        success(t("✓ 编译成功", "✓ Build succeeded"));
    }
    if let Err(e) = &result {
        if !is_simulator_destination(&config.destination) {
            anyhow::bail!(
                "{e}\n\n{}",
                t(
                    "真机构建需要代码签名：\n\
                     1. 用 Xcode 打开工程 → Target → Signing & Capabilities → 选择 Team\n\
                     2. 或在全局配置 ~/.config/ios-runner/config.toml 为该工程设置 development_team",
                    "Device builds require code signing:\n\
                     1. Open the project in Xcode → Target → Signing & Capabilities → select a Team\n\
                     2. Or set development_team in ~/.config/ios-runner/config.toml for this project",
                )
            );
        }
    }
    result
}

pub fn run_app(root: &Path, config: &RunnerConfig) -> Result<()> {
    if is_simulator_destination(&config.destination) {
        run_on_simulator(root, config)
    } else {
        run_on_device(root, config)
    }
}

pub fn run_on_simulator(root: &Path, config: &RunnerConfig) -> Result<()> {
    build_project(root, config)?;

    let artifacts = launch_artifacts(root, config)?;
    let app_path = artifacts.app_path;
    let bundle_id = artifacts.bundle_identifier;

    let device_udid = simulator_udid_for_destination(&config.destination)?;
    boot_simulator(&device_udid)?;

    if config.bring_simulator_to_foreground {
        let _ = Command::new("open").args(["-a", "Simulator"]).status();
    }

    let app = app_path
        .to_str()
        .context("app path is not valid UTF-8")?;

    let status = Command::new("xcrun")
        .args(["simctl", "install", &device_udid, app])
        .status()
        .context("simctl install")?;
    if !status.success() {
        bail!("simctl install failed");
    }

    section(
        t("应用日志", "App log"),
        Some(t(
            "点击 App 内按钮，输出会显示在下方 · Ctrl+C 结束",
            "Tap buttons in the app to see output below · Ctrl+C to stop",
        )),
    );
    info(&format!("{} {bundle_id}", t("启动", "Launch")));
    let status = Command::new("xcrun")
        .args([
            "simctl",
            "launch",
            "--console-pty",
            "--terminate-running-process",
            &device_udid,
            &bundle_id,
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("simctl launch")?;
    if !status.success() {
        bail!("simctl launch failed");
    }

    success(t("✓ 已启动（模拟器）", "✓ Launched on simulator"));
    Ok(())
}

pub fn run_on_device(root: &Path, config: &RunnerConfig) -> Result<()> {
    build_project(root, config)?;

    let artifacts = launch_artifacts(root, config)?;
    let app_path = artifacts
        .app_path
        .to_str()
        .context("app path is not valid UTF-8")?;
    let bundle_id = artifacts.bundle_identifier;
    let device_id = device_udid_from_destination(&config.destination)?;
    ensure_device_ready(&device_id)?;

    section(
        t("安装到真机", "Install on device"),
        Some(&config.device_summary()),
    );
    info(&format!(
        "{} {device_id}",
        t("设备安装", "Installing on device"),
    ));
    let install = run_devicectl(&[
        "device",
        "install",
        "app",
        "--device",
        &device_id,
        app_path,
    ])?;
    if !install.status.success() {
        let stderr = String::from_utf8_lossy(&install.stderr);
        let stdout = String::from_utf8_lossy(&install.stdout);
        if !stderr.trim().is_empty() {
            info(stderr.trim());
        }
        report_devicectl_failure(t("安装到真机", "Install on device"), &stderr, &stdout)?;
    }

    ensure_device_ready(&device_id)?;

    section(
        t("应用日志", "App log"),
        Some(t("Ctrl+C 结束", "Ctrl+C to stop")),
    );
    info(&format!("{} {bundle_id}", t("启动", "Launch")));
    let launch_status = Command::new("xcrun")
        .args([
            "devicectl",
            "device",
            "process",
            "launch",
            "--device",
            &device_id,
            "--console",
            "--terminate-existing",
            &bundle_id,
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("devicectl launch")?;
    if !launch_status.success() {
        let _ = ensure_device_ready(&device_id);
        warn_device_launch_hints();
        bail!("{}", t("启动应用失败", "Failed to launch app"));
    }

    success(t("✓ 已启动（真机）", "✓ Launched on device"));
    Ok(())
}

fn run_devicectl(args: &[&str]) -> Result<std::process::Output> {
    Command::new("xcrun")
        .arg("devicectl")
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("devicectl")
}

fn warn_device_launch_hints() {
    warn(t(
        "若 iPhone 已锁屏，请先解锁并保持亮屏；确认已在手机上信任此 Mac，且已开启「开发者模式」。",
        "If the iPhone is locked, unlock it and keep the screen on; trust this Mac and enable Developer Mode.",
    ));
}

fn resolve_packages_quiet(root: &Path, config: &RunnerConfig) -> Result<()> {
    let mut cmd = Command::new("xcodebuild");
    add_config_args(&mut cmd, config);
    cmd.arg("-resolvePackageDependencies");
    let status = cmd
        .current_dir(root)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .status()
        .context("resolve packages")?;
    if status.success() {
        Ok(())
    } else {
        bail!("resolvePackageDependencies failed (exit {:?})", status.code())
    }
}

fn simulator_udid_for_destination(destination: &str) -> Result<String> {
    let name = destination
        .split(',')
        .find_map(|part| {
            let part = part.trim();
            part.strip_prefix("name=").map(|s| s.trim())
        })
        .context("simulator destination missing name=")?;
    udid_for_destination_name(name)
}

fn boot_simulator(udid: &str) -> Result<()> {
    let _ = Command::new("xcrun")
        .args(["simctl", "boot", udid])
        .status();
    Ok(())
}

pub(crate) fn add_project_args(cmd: &mut Command, project: &DetectedProject) {
    match project.kind {
        ProjectKind::Workspace => {
            cmd.args(["-workspace", &project.path.to_string_lossy()]);
        }
        ProjectKind::Project => {
            cmd.args(["-project", &project.path.to_string_lossy()]);
        }
    }
}

pub(crate) fn add_config_args(cmd: &mut Command, config: &RunnerConfig) {
    match config.kind {
        ProjectKind::Workspace => {
            cmd.args(["-workspace", &config.path]);
        }
        ProjectKind::Project => {
            cmd.args(["-project", &config.path]);
        }
    }
    cmd.args(["-scheme", &config.scheme]);
}

fn run_command(mut cmd: Command, root: &Path, label: &str) -> Result<()> {
    let _ = label;
    let status = cmd
        .current_dir(root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| label.to_string())?;
    if status.success() {
        Ok(())
    } else {
        bail!("{label} failed (exit {:?})", status.code())
    }
}

fn which_xcbeautify() -> bool {
    Command::new("xcbeautify")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Pipe xcodebuild stdout/stderr through xcbeautify (SweetPad default when installed).
fn run_xcodebuild_piped(mut cmd: Command, root: &Path) -> Result<()> {
    info(t("xcodebuild → xcbeautify", "xcodebuild → xcbeautify"));

    cmd.current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut xcodebuild = cmd.spawn().context("spawn xcodebuild")?;

    let xcode_stdout = xcodebuild
        .stdout
        .take()
        .context("xcodebuild stdout pipe missing")?;

    let mut beauty = Command::new("xcbeautify")
        .stdin(xcode_stdout)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("spawn xcbeautify")?;

    if let Some(mut stderr) = xcodebuild.stderr.take() {
        let mut err_out = io::stderr();
        std::thread::spawn(move || {
            let _ = io::copy(&mut stderr, &mut err_out);
        });
    }

    let xcode_status = xcodebuild.wait().context("wait xcodebuild")?;
    let beauty_status = beauty.wait().context("wait xcbeautify")?;

    if xcode_status.success() && beauty_status.success() {
        Ok(())
    } else {
        bail!(
            "build failed (xcodebuild {:?}, xcbeautify {:?})",
            xcode_status.code(),
            beauty_status.code()
        );
    }
}

#[derive(Debug, Deserialize)]
struct ListJson {
    project: Option<ProjectList>,
    workspace: Option<WorkspaceList>,
}

#[derive(Debug, Deserialize)]
struct ProjectList {
    schemes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct WorkspaceList {
    schemes: Option<Vec<String>>,
}

