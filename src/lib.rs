use zed_extension_api::{
    current_platform, download_file, github_release_by_tag_name, make_file_executable,
    process::Command as ProcessCommand, Architecture, ContextServerId, DownloadedFileType,
    Extension, Project, Result,
};

struct IosRunnerExtension;

impl Extension for IosRunnerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<ProcessCommand> {
        let path = ensure_mcp_binary()?;
        Ok(ProcessCommand::new(&path).args(["mcp"]))
    }
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

/// Use a platform CLI already present in the extension checkout/package.
fn try_bundled_cli() -> Result<String, String> {
    let (_os, arch) = current_platform();
    let suffix = arch_asset_suffix(arch)?;
    let name = format!("ios-runner-{suffix}");
    let candidates = [
        bundled_cli_relative(suffix),
        format!("../bin/{name}"),
        format!("../../bin/{name}"),
    ];
    for bundled in &candidates {
        if cli_installation_looks_valid(bundled) {
            return Ok(bundled.to_string());
        }
    }
    Err("bundled CLI not found in extension package".to_string())
}

/// Download the MCP server binary into the extension work directory when it is not bundled.
fn download_cli_from_github_release() -> Result<String, String> {
    let (_os, arch) = current_platform();
    let suffix = arch_asset_suffix(arch)?;
    let asset_name = format!("ios-runner-{suffix}");
    let tag = format!("v{}", env!("CARGO_PKG_VERSION"));
    let release = github_release_by_tag_name("buds520/ios-runner", &tag)?;

    let asset = release
        .assets
        .into_iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| format!("release asset not found: {asset_name}"))?;

    let dest = bundled_cli_relative(suffix);
    if !cli_installation_looks_valid(&dest) {
        download_file(&asset.download_url, &dest, DownloadedFileType::Uncompressed)?;
        make_file_executable(&dest)?;
    }
    Ok(dest)
}

fn ensure_mcp_binary() -> Result<String, String> {
    try_bundled_cli().or_else(|_| download_cli_from_github_release())
}

zed_extension_api::register_extension!(IosRunnerExtension);
