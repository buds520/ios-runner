use std::path::PathBuf;

use zed_extension_api::{
    Architecture, ContextServerId, DownloadedFileType, Extension, GithubReleaseOptions, Project,
    Result, current_platform, download_file, latest_github_release, make_file_executable,
    process::Command as ProcessCommand,
};

struct IosRunnerExtension;

const EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");

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

fn bootstrap_marker_path() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(format!(".ios-runner/.bootstrap-v{EXTENSION_VERSION}")))
}

/// On extension load: download CLI (if needed), install to ~/.ios-runner/bin, refresh global Zed tasks (once per version).
fn bootstrap_install() -> Result<(), String> {
    if let Some(marker) = bootstrap_marker_path() {
        if marker.is_file() {
            return Ok(());
        }
    }

    let path = ensure_cli_binary()?;
    if path == "ios-runner" {
        return Ok(());
    }

    let install_bin = if path.contains("/.ios-runner/bin/ios-runner") {
        path.clone()
    } else if let Ok(home) = std::env::var("HOME") {
        let install_dir = format!("{home}/.ios-runner/bin");
        let install_bin = format!("{install_dir}/ios-runner");
        ProcessCommand::new("/bin/mkdir")
            .args(["-p", &install_dir])
            .output()
            .map_err(|e| e.to_string())?;
        ProcessCommand::new("/bin/cp")
            .args([&path, &install_bin])
            .output()
            .map_err(|e| e.to_string())?;
        ProcessCommand::new("/bin/chmod")
            .args(["+x", &install_bin])
            .output()
            .map_err(|e| e.to_string())?;
        install_bin
    } else {
        return Ok(());
    };

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
    if let Ok(home) = std::env::var("HOME") {
        let installed = PathBuf::from(home).join(".ios-runner/bin/ios-runner");
        if installed.is_file() {
            return Ok(installed.to_string_lossy().into_owned());
        }
    }

    if let Ok(path) = try_download_release() {
        return Ok(path);
    }

    Ok("ios-runner".to_string())
}

fn try_download_release() -> Result<String, String> {
    let (_os, arch) = current_platform();
    let arch_name = match arch {
        Architecture::Aarch64 => "aarch64-apple-darwin",
        Architecture::X8664 => "x86_64-apple-darwin",
        other => return Err(format!("unsupported architecture: {other:?}")),
    };

    let asset_name = format!("ios-runner-{arch_name}");
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

    let dest = format!("bin/ios-runner-{arch_name}");
    download_file(&asset.download_url, &dest, DownloadedFileType::Uncompressed)?;
    make_file_executable(&dest)?;

    if let Ok(home) = std::env::var("HOME") {
        let installed = PathBuf::from(home).join(".ios-runner/bin/ios-runner");
        if installed.is_file() {
            return Ok(installed.to_string_lossy().into_owned());
        }
    }

    Ok(dest)
}

zed_extension_api::register_extension!(IosRunnerExtension);
