use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::build_diagnostics::append_and_persist;
use crate::build_settings::launch_artifacts;
use crate::config::{ProjectKind, RunnerConfig};
use crate::detect::DetectedProject;
use crate::destination::{
    DestinationKind, device_udid_from_destination, is_macos_destination,
    is_simulator_destination, list_run_destinations,
};
use crate::device::{ensure_device_ready, report_devicectl_failure};
use crate::simulator::{destination_for_simulator, list_simulators, udid_for_destination_name};
use crate::locale::t;
use crate::terminal_ui::{info, print_project_context, section, success, warn};

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
    let destinations = list_run_destinations(root, project, scheme)?;

    if let Some(d) = destinations
        .iter()
        .find(|d| d.kind == DestinationKind::Simulator && d.name.starts_with("iPhone"))
    {
        return Ok(d.destination.clone());
    }

    if let Some(d) = destinations
        .iter()
        .find(|d| d.kind == DestinationKind::Simulator)
    {
        return Ok(d.destination.clone());
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

pub fn resolve_packages(root: &Path, config: &RunnerConfig) -> Result<()> {
    let mut cmd = Command::new("xcodebuild");
    add_config_args(&mut cmd, config);
    cmd.arg("-resolvePackageDependencies");

    run_command(cmd, root, "resolve package dependencies")
}

pub fn build_project(root: &Path, config: &RunnerConfig) -> Result<()> {
    config.validate(root)?;
    print_project_context(config);

    if should_skip_incremental_build(root, config)? {
        success(t(
            "✓ 源码未变化，跳过编译（设置 IOS_RUNNER_FORCE_BUILD=1 强制编译）",
            "✓ Sources unchanged, skipping build (set IOS_RUNNER_FORCE_BUILD=1 to force)",
        ));
        return Ok(());
    }

    section(
        t("编译", "Build"),
        Some(&format!("{} · {}", config.scheme, config.device_summary())),
    );
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

    let has_xcbeautify = crate::has_xcbeautify();
    let result = if config.xcbeautify && has_xcbeautify && !raw_logs {
        run_xcodebuild_piped(cmd, root)
    } else {
        if raw_logs {
            info(t(
                "完整 xcodebuild 日志（IOS_RUNNER_RAW_LOG=1）",
                "Full xcodebuild log (IOS_RUNNER_RAW_LOG=1)",
            ));
        } else {
            crate::terminal_ui::warn_xcbeautify_missing(config.xcbeautify);
        }
        run_command(cmd, root, "build")
    };
    if result.is_ok() {
        success(t("✓ 编译成功", "✓ Build succeeded"));
    }
    if let Err(e) = &result {
        if !is_simulator_destination(&config.destination) && !is_macos_destination(&config.destination) {
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

fn build_for_run(root: &Path, config: &RunnerConfig) -> Result<()> {
    if should_skip_incremental_build(root, config)? {
        if launch_artifacts(root, config).is_ok() {
            success(t(
                "✓ 源码未变化，跳过编译",
                "✓ Sources unchanged, skipping build",
            ));
            return Ok(());
        }
        warn(t(
            "未找到 .app，将重新编译",
            ".app not found, rebuilding",
        ));
        std::env::set_var("IOS_RUNNER_FORCE_BUILD", "1");
    }
    build_project(root, config)
}

fn force_build_requested() -> bool {
    std::env::var("IOS_RUNNER_FORCE_BUILD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn skip_if_fresh_enabled() -> bool {
    std::env::var("IOS_RUNNER_SKIP_IF_FRESH")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn should_skip_incremental_build(root: &Path, config: &RunnerConfig) -> Result<bool> {
    if force_build_requested() || !skip_if_fresh_enabled() {
        return Ok(false);
    }
    detect_incremental_fresh(root, config)
}

/// Compare latest source mtime under `root` with newest xcactivitylog in DerivedData.
pub fn detect_incremental_fresh(root: &Path, config: &RunnerConfig) -> Result<bool> {
    let Some(source_mtime) = latest_source_mtime(root)? else {
        return Ok(false);
    };
    let derived = config.derived_data_path(root);
    let Some(log_mtime) = latest_xcactivitylog_mtime(&derived)? else {
        return Ok(false);
    };
    Ok(source_mtime <= log_mtime)
}

fn walk_skip_dir(name: &str) -> bool {
    matches!(
        name,
        "Pods" | "DerivedData" | ".build" | ".git" | ".ios-runner" | "node_modules"
            | ".bluecode" | "xcuserdata"
    )
}

fn latest_source_mtime(root: &Path) -> Result<Option<SystemTime>> {
    let mut latest: Option<SystemTime> = None;
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|n| !walk_skip_dir(n))
                .unwrap_or(true)
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        if path.file_name().is_some_and(|n| n == "Info.plist")
            || path.extension().is_some_and(|e| e == "pbxproj")
        {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !matches!(
            ext,
            "swift" | "m" | "mm" | "h" | "c" | "cpp" | "metal" | "storyboard" | "xib"
        ) {
            continue;
        }
        if let Ok(mtime) = path.metadata().and_then(|m| m.modified()) {
            latest = Some(match latest {
                Some(prev) if mtime > prev => mtime,
                Some(prev) => prev,
                None => mtime,
            });
        }
    }
    Ok(latest)
}

fn latest_xcactivitylog_mtime(derived: &Path) -> Result<Option<SystemTime>> {
    if !derived.is_dir() {
        return Ok(None);
    }
    let logs = derived.join("Logs/Build");
    if !logs.is_dir() {
        return Ok(None);
    }
    let mut latest: Option<SystemTime> = None;
    for entry in WalkDir::new(&logs).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("xcactivitylog") {
            continue;
        }
        if path.metadata().map(|m| m.len()).unwrap_or(0) == 0 {
            continue;
        }
        if let Ok(mtime) = path.metadata().and_then(|m| m.modified()) {
            latest = Some(match latest {
                Some(prev) if mtime > prev => mtime,
                Some(prev) => prev,
                None => mtime,
            });
        }
    }
    Ok(latest)
}

pub fn run_app(root: &Path, config: &RunnerConfig) -> Result<()> {
    if is_macos_destination(&config.destination) {
        run_on_mac(root, config)
    } else if is_simulator_destination(&config.destination) {
        run_on_simulator(root, config)
    } else {
        run_on_device(root, config)
    }
}

pub fn run_on_simulator(root: &Path, config: &RunnerConfig) -> Result<()> {
    build_for_run(root, config)?;

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
            "点击 App 内按钮，输出会显示在下方 · Ctrl+C 结束日志（并停止 App）",
            "Tap buttons in the app to see output below · Ctrl+C stops logging (and the app)",
        )),
    );
    info(&format!("{} {bundle_id}", t("启动", "Launch")));
    // exec replaces this process so Ctrl+C reaches simctl (works in Zed tasks too).
    exec_inherited("xcrun", [
        "simctl",
        "launch",
        "--console-pty",
        "--terminate-running-process",
        &device_udid,
        &bundle_id,
    ])
}

pub fn run_on_device(root: &Path, config: &RunnerConfig) -> Result<()> {
    build_for_run(root, config)?;

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
    if let Err(e) = exec_inherited(
        "xcrun",
        [
            "devicectl",
            "device",
            "process",
            "launch",
            "--device",
            &device_id,
            "--console",
            "--terminate-existing",
            &bundle_id,
        ],
    ) {
        let _ = ensure_device_ready(&device_id);
        warn_device_launch_hints();
        return Err(e);
    }
    Ok(())
}

pub fn run_on_mac(root: &Path, config: &RunnerConfig) -> Result<()> {
    build_for_run(root, config)?;

    let artifacts = launch_artifacts(root, config)?;
    let app_path = artifacts.app_path;

    section(
        t("启动 Mac 应用", "Launch Mac app"),
        Some(&config.device_summary()),
    );
    info(&format!(
        "{} {}",
        t("打开", "Open"),
        app_path.display()
    ));

    let status = Command::new("open")
        .arg(&app_path)
        .status()
        .context("open app")?;
    if !status.success() {
        bail!("open failed (exit {:?})", status.code());
    }

    success(t("✓ 应用已启动", "✓ App launched"));
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
    let output = Command::new("xcrun")
        .args(["simctl", "boot", udid])
        .output()
        .context("simctl boot")?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("current state: Booted") || stderr.contains("Unable to boot device in current state: Booted")
    {
        return Ok(());
    }
    if !stderr.trim().is_empty() {
        warn(stderr.trim());
    }
    Ok(())
}

/// Run a foreground tool with inherited stdio; on macOS use exec so Ctrl+C stops the child directly.
fn exec_inherited(program: &str, args: impl IntoIterator<Item = impl AsRef<std::ffi::OsStr>>) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let label = format!("{program} (exec)");
        let err = cmd.exec();
        bail!("{label} failed: {err}");
    }

    #[cfg(not(unix))]
    {
        let status = cmd.status().with_context(|| format!("{program}"))?;
        if status.success() {
            Ok(())
        } else {
            bail!("{program} failed (exit {:?})", status.code())
        }
    }
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
        bail!("{}", xcodebuild_failure_message(label, status.code()))
    }
}

fn xcodebuild_failure_message(label: &str, code: Option<i32>) -> String {
    crate::locale::tf(
        || format!("{label} 失败 (exit {code:?})"),
        || format!("{label} failed (exit {code:?})"),
    )
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

    let stderr_capture = Arc::new(Mutex::new(Vec::new()));
    if let Some(mut stderr) = xcodebuild.stderr.take() {
        let cap = Arc::clone(&stderr_capture);
        std::thread::spawn(move || {
            use std::io::{Read, Write};
            let mut err_out = io::stderr();
            let mut chunk = [0u8; 8192];
            loop {
                match stderr.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(n) => {
                        let _ = err_out.write_all(&chunk[..n]);
                        if let Ok(mut guard) = cap.lock() {
                            guard.extend_from_slice(&chunk[..n]);
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }

    let xcode_status = xcodebuild.wait().context("wait xcodebuild")?;
    let beauty_status = beauty.wait().context("wait xcbeautify")?;

    if xcode_status.success() && beauty_status.success() {
        Ok(())
    } else {
        let stderr = stderr_capture
            .lock()
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let _ = append_and_persist(&stderr, "");
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

