use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::build_settings::launch_artifacts;
use crate::config::{ProjectKind, RunnerConfig};
use crate::detect::DetectedProject;
use crate::simulator::udid_for_destination_name;

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

    Ok("platform=iOS Simulator,name=iPhone 16".to_string())
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
    if config.resolve_packages_before_build {
        let _ = resolve_packages(root, config);
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
        "build",
    ]);

    if config.xcbeautify && which_xcbeautify() {
        run_xcodebuild_piped(cmd, root)
    } else {
        run_command(cmd, root, "build")
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

    // SweetPad: simctl launch --console-pty --terminate-running-process
    let status = Command::new("xcrun")
        .args([
            "simctl",
            "launch",
            "--console-pty",
            "--terminate-running-process",
            &device_udid,
            &bundle_id,
        ])
        .status()
        .context("simctl launch")?;
    if !status.success() {
        bail!("simctl launch failed");
    }

    eprintln!("Launched {bundle_id} on simulator {device_udid}");
    Ok(())
}

fn simulator_udid_for_destination(destination: &str) -> Result<String> {
    let name = destination
        .split(',')
        .find_map(|part| {
            let part = part.trim();
            part.strip_prefix("name=").map(|s| s.trim())
        })
        .unwrap_or("iPhone 16");
    udid_for_destination_name(name)
}

fn boot_simulator(udid: &str) -> Result<()> {
    let _ = Command::new("xcrun")
        .args(["simctl", "boot", udid])
        .status();
    Ok(())
}

fn add_project_args(cmd: &mut Command, project: &DetectedProject) {
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
    eprintln!("→ {label}: {cmd:?}");
    let status = cmd.current_dir(root).status().with_context(|| label.to_string())?;
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
    eprintln!("→ build (via xcbeautify): {cmd:?}");

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

