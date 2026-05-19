use std::path::PathBuf;

use zed_extension_api::{
    Architecture, ContextServerId, DownloadedFileType, Extension, GithubReleaseOptions, Project,
    Result, current_platform, download_file, latest_github_release, make_file_executable,
    process::Command as ProcessCommand,
};

struct IosRunnerExtension;

const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Bump when Zed task scripts change (forces `install-zed-tasks` even if extension version unchanged).
const TASKS_SCHEMA: &str = "tasks-v6-quoted-home";

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

/// On extension load: install CLI + refresh global Zed tasks (when CLI or task schema changes).
fn bootstrap_install() -> Result<(), String> {
    let install_bin = install_cli_to_home()?;

    if let Some(marker) = bootstrap_marker_path() {
        if marker.is_file() && cli_installation_looks_valid(&install_bin) {
            return Ok(());
        }
    }

    let out = ProcessCommand::new(&install_bin)
        .args(["install-zed-tasks"])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status != Some(0) {
        return Err(format!(
            "install-zed-tasks failed (exit {:?}): {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        ));
    }

    if let Some(marker) = bootstrap_marker_path() {
        if let Some(parent) = marker.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&marker, EXTENSION_VERSION);
    }

    Ok(())
}

fn ensure_cli_binary() -> Result<String, String> {
    if let Ok(path) = install_cli_to_home() {
        if std::path::Path::new(&path).is_file() {
            return Ok(path);
        }
    }

    // Last resort: hope `ios-runner` is on PATH (e.g. cargo install during development).
    Ok("ios-runner".to_string())
}

zed_extension_api::register_extension!(IosRunnerExtension);
