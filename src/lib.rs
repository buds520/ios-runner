use std::path::PathBuf;

use zed_extension_api::{
    Architecture, ContextServerId, DownloadedFileType, Extension, GithubReleaseOptions, Project,
    Result, current_platform, download_file, latest_github_release, make_file_executable,
    process::Command as ProcessCommand,
};

struct IosRunnerExtension;

const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Bump when Zed task scripts change (forces `install-zed-tasks` even if extension version unchanged).
const TASKS_SCHEMA: &str = "tasks-v16-in-panel-terminal";
const TASK_LABEL_PREFIX: &str = "iOS-Runner:";
const EMBEDDED_GLOBAL_TASKS: &str = include_str!("embedded_global_tasks.json");
const EMBEDDED_KEYMAP_ENTRY: &str = include_str!("embedded_keymap_entry.json");

impl Extension for IosRunnerExtension {
    fn new() -> Self {
        if let Err(e) = bootstrap_install() {
            eprintln!("[ios-runner] bootstrap: {e}");
        }
        Self
    }

    fn context_server_command(
        &mut self,
        _server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<ProcessCommand> {
        let path = ensure_cli_binary()?;
        Ok(ProcessCommand::new(&path).args(["mcp"]))
    }
}

fn home_install_bin() -> Option<String> {
    std::env::var("HOME")
        .ok()
        .map(|home| format!("{home}/.ios-runner/bin/ios-runner"))
}

fn bootstrap_marker_path() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(|home| {
        PathBuf::from(home).join(format!(
            ".ios-runner/.bootstrap-{EXTENSION_VERSION}-{TASKS_SCHEMA}"
        ))
    })
}

/// Release CLI is ~2MB; smaller files are usually failed curl/HTML downloads.
fn cli_installation_looks_valid(path: &str) -> bool {
    std::path::Path::new(path)
        .metadata()
        .map(|m| m.is_file() && m.len() > 500_000)
        .unwrap_or(false)
}

fn arch_asset_suffix(arch: Architecture) -> Result<&'static str, String> {
    match arch {
        Architecture::Aarch64 => Ok("aarch64-apple-darwin"),
        Architecture::X8664 => Ok("x86_64-apple-darwin"),
        other => Err(format!("unsupported architecture: {other:?}")),
    }
}

/// Relative path inside the extension package (same layout as `download_file` dest).
fn bundled_cli_relative(arch_suffix: &str) -> String {
    format!("bin/ios-runner-{arch_suffix}")
}

fn copy_file_executable(src: &str, dest: &str) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(dest).parent() {
        ProcessCommand::new("/bin/mkdir")
            .args(["-p", &parent.display().to_string()])
            .output()
            .map_err(|e| e.to_string())?;
    }
    let out = ProcessCommand::new("/bin/cp")
        .args([src, dest])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status != Some(0) {
        return Err(format!(
            "cp failed ({} → {}): {}",
            src,
            dest,
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let chmod = ProcessCommand::new("/bin/chmod")
        .args(["+x", dest])
        .output()
        .map_err(|e| e.to_string())?;
    if chmod.status != Some(0) {
        return Err(format!(
            "chmod failed: {}",
            String::from_utf8_lossy(&chmod.stderr)
        ));
    }
    Ok(())
}

/// Copy platform CLI from extension `bin/` into `~/.ios-runner/bin/ios-runner`.
fn try_install_from_extension_bundle(install_bin: &str) -> Result<(), String> {
    let (_os, arch) = current_platform();
    let suffix = arch_asset_suffix(arch)?;
    let name = format!("ios-runner-{suffix}");
    let candidates = [
        bundled_cli_relative(suffix),
        format!("../bin/{name}"),
        format!("../../bin/{name}"),
    ];
    for bundled in &candidates {
        if copy_file_executable(bundled, install_bin).is_ok() {
            return Ok(());
        }
    }
    Err("bundled CLI not found in extension package".to_string())
}

/// Fallback when the extension package has no `bin/` (dev checkout without bundle step).
fn try_install_from_github_release(install_bin: &str) -> Result<(), String> {
    let (_os, arch) = current_platform();
    let suffix = arch_asset_suffix(arch)?;
    let asset_name = format!("ios-runner-{suffix}");
    let release = latest_github_release(
        "buds520/ios-runner",
        GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    let asset = release
        .assets
        .into_iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| format!("release asset not found: {asset_name}"))?;

    let dest = bundled_cli_relative(suffix);
    download_file(&asset.download_url, &dest, DownloadedFileType::Uncompressed)?;
    make_file_executable(&dest)?;
    copy_file_executable(&dest, install_bin)
}

/// Ensure `~/.ios-runner/bin/ios-runner` exists: bundled CLI first, then GitHub Release.
fn install_cli_to_home() -> Result<String, String> {
    let install_bin = home_install_bin().ok_or_else(|| "HOME not set".to_string())?;

    if cli_installation_looks_valid(&install_bin) {
        return Ok(install_bin);
    }

    if try_install_from_extension_bundle(&install_bin).is_ok() {
        return Ok(install_bin);
    }

    try_install_from_github_release(&install_bin)?;
    Ok(install_bin)
}

/// True when `~/.config/zed/tasks.json` is missing or has no current iOS-Runner tasks.
fn global_tasks_need_refresh() -> bool {
    let Some(home) = std::env::var("HOME").ok() else {
        return true;
    };
    let path = std::path::Path::new(&home).join(".config/zed/tasks.json");
    if !path.is_file() {
        return true;
    }
    let Ok(text) = std::fs::read_to_string(&path) else {
        return true;
    };
    if text.contains("curl -fsSL")
        || text.contains("$ir_bin")
        || text.contains("$IOS_RUNNER")
        || text.contains("ios-runner install-self")
        || text.contains("尚未就绪")
        || text.contains("not ready yet")
    {
        return true;
    }
    !(text.contains("iOS-Runner: Run")
        || text.contains("iOS-Runner: 运行")
        || text.contains("iOS-Runner: 初始化项目")
        || text.contains("iOS-Runner: Initialize Project")
        || text.contains("iOS-Runner: Setup Project"))
}

/// Write global tasks from the WASM bundle (no CLI required).
fn install_embedded_global_tasks() -> Result<(), String> {
    if !global_tasks_need_refresh() {
        return Ok(());
    }

    let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
    let path = PathBuf::from(&home).join(".config/zed/tasks.json");

    let embedded: Vec<serde_json::Value> =
        serde_json::from_str(EMBEDDED_GLOBAL_TASKS).map_err(|e| e.to_string())?;

    let mut tasks: Vec<serde_json::Value> = if path.is_file() {
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&text).unwrap_or_default()
    } else {
        Vec::new()
    };

    tasks.retain(|t| !task_is_ios_runner(t));
    tasks.extend(embedded);

    let text = serde_json::to_string_pretty(&tasks).map_err(|e| e.to_string())?;
    write_user_config_file(&path, &text)
}

fn global_keymap_needs_refresh() -> bool {
    let Some(home) = std::env::var("HOME").ok() else {
        return true;
    };
    let path = std::path::Path::new(&home).join(".config/zed/keymap.json");
    if !path.is_file() {
        return true;
    }
    let Ok(text) = std::fs::read_to_string(&path) else {
        return true;
    };
    !text.contains("iOS-Runner:")
}

fn install_embedded_global_keymap() -> Result<(), String> {
    if !global_keymap_needs_refresh() {
        return Ok(());
    }

    let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
    let path = PathBuf::from(&home).join(".config/zed/keymap.json");

    let ours: serde_json::Value =
        serde_json::from_str(EMBEDDED_KEYMAP_ENTRY).map_err(|e| e.to_string())?;

    let mut entries: Vec<serde_json::Value> = if path.is_file() {
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&text).unwrap_or_default()
    } else {
        Vec::new()
    };

    let ours_bindings = ours
        .get("bindings")
        .and_then(|b| b.as_object())
        .ok_or_else(|| "invalid embedded keymap".to_string())?;

    if let Some(workspace) = entries
        .iter_mut()
        .find(|e| e.get("context") == Some(&serde_json::json!("Workspace")))
    {
        let bindings = workspace
            .as_object_mut()
            .and_then(|o| o.get_mut("bindings"))
            .and_then(|b| b.as_object_mut())
            .ok_or_else(|| "invalid keymap workspace entry".to_string())?;
        for (k, v) in ours_bindings {
            bindings.entry(k.clone()).or_insert(v.clone());
        }
    } else {
        entries.push(ours);
    }

    let text = serde_json::to_string_pretty(&entries).map_err(|e| e.to_string())?;
    write_user_config_file(&path, &text)
}

/// Write under `~/.config/zed/`; fall back to `/usr/bin/python3` if direct fs fails in WASM.
fn write_user_config_file(path: &std::path::Path, text: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if std::fs::create_dir_all(parent).is_err() {
            let _ = ProcessCommand::new("/bin/mkdir")
                .args(["-p", &parent.display().to_string()])
                .output();
        }
    }

    if std::fs::write(path, text).is_ok() {
        return Ok(());
    }

    let out = ProcessCommand::new("/usr/bin/python3")
        .args([
            "-c",
            "import pathlib, sys; pathlib.Path(sys.argv[1]).write_text(sys.argv[2])",
            &path.display().to_string(),
            text,
        ])
        .output()
        .map_err(|e| format!("python3 write failed: {e}"))?;
    if out.status == Some(0) {
        return Ok(());
    }
    Err(format!(
        "write {} failed: {}",
        path.display(),
        String::from_utf8_lossy(&out.stderr)
    ))
}

fn task_is_ios_runner(task: &serde_json::Value) -> bool {
    task.get("label")
        .and_then(|l| l.as_str())
        .is_some_and(|l| l.starts_with(TASK_LABEL_PREFIX))
}

/// On extension load: embedded global tasks + keymap; CLI install is best-effort.
fn bootstrap_install() -> Result<(), String> {
    install_embedded_global_tasks()?;
    install_embedded_global_keymap()?;

    match install_cli_to_home() {
        Ok(install_bin) => {
            let out = ProcessCommand::new(&install_bin)
                .args(["install-zed-tasks"])
                .output()
                .map_err(|e| e.to_string())?;
            if out.status != Some(0) {
                eprintln!(
                    "[ios-runner] install-zed-tasks skipped (exit {:?}): {}",
                    out.status,
                    String::from_utf8_lossy(&out.stderr)
                );
            }
        }
        Err(e) => {
            eprintln!("[ios-runner] CLI install deferred: {e}");
            print_dev_extension_hint();
        }
    }

    if let Some(marker) = bootstrap_marker_path() {
        if let Some(parent) = marker.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&marker, EXTENSION_VERSION);
    }

    Ok(())
}

fn print_dev_extension_hint() {
    eprintln!(
        r"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  iOS-Runner Dev Extension
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Dev Extension 需要单独安装 CLI，任务面板才会完整可用。

【方式一】一键安装（推荐）
  curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/install-dev.sh | bash

【方式二】手动
  cd /path/to/ios-runner/crates && cargo build -p ios-runner-cli --release
  cp target/release/ios-runner ~/.ios-runner/bin/
  ios-runner install-zed-tasks

安装后 Cmd+Q 退出 Zed 并重新打开。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"
    );
}

fn ensure_cli_binary() -> Result<String, String> {
    if let Ok(path) = install_cli_to_home() {
        if std::path::Path::new(&path).is_file() {
            return Ok(path);
        }
    }

    Ok("ios-runner".to_string())
}

zed_extension_api::register_extension!(IosRunnerExtension);
