use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::config::RunnerConfig;
use crate::destination::is_simulator_destination;

/// Launch-related paths from `xcodebuild -showBuildSettings -json`.
/// Aligned with SweetPad's `XcodeBuildSettings` in `common/cli/scripts.ts`.
#[derive(Debug, Clone)]
pub struct LaunchArtifacts {
    pub app_path: PathBuf,
    pub bundle_identifier: String,
}

pub fn launch_artifacts(root: &Path, config: &RunnerConfig) -> Result<LaunchArtifacts> {
    let derived = config.derived_data_path(root);
    let sdk = if is_simulator_destination(&config.destination) {
        "iphonesimulator"
    } else {
        "iphoneos"
    };
    let mut cmd = Command::new("xcodebuild");
    super::xcodebuild::add_config_args(&mut cmd, config);
    cmd.args([
        "-configuration",
        &config.configuration,
        "-destination",
        &config.destination,
        "-sdk",
        sdk,
        "-showBuildSettings",
        "-json",
    ]);
    if derived.exists() || std::fs::create_dir_all(&derived).is_ok() {
        cmd.args(["-derivedDataPath", &derived.to_string_lossy()]);
    }

    let output = cmd
        .current_dir(root)
        .output()
        .context("xcodebuild -showBuildSettings")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("showBuildSettings failed:\n{stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_text = extract_json_payload(&stdout).context("no JSON in showBuildSettings output")?;
    let entries: Vec<BuildSettingsEntry> =
        serde_json::from_str(&json_text).context("parse showBuildSettings JSON")?;

    let entry = entries
        .first()
        .context("empty showBuildSettings (wrong scheme or destination?)")?;

    let settings = &entry.build_settings;
    let target_build_dir = settings
        .target_build_dir
        .as_ref()
        .context("TARGET_BUILD_DIR missing")?;
    let app_name: String = if let Some(name) = &settings.wrapper_name {
        name.clone()
    } else if let Some(name) = &settings.full_product_name {
        name.clone()
    } else if let Some(name) = &settings.product_name {
        format!("{name}.app")
    } else {
        bail!("WRAPPER_NAME / FULL_PRODUCT_NAME / PRODUCT_NAME missing in build settings");
    };

    let app_path = PathBuf::from(target_build_dir).join(app_name);
    if !app_path.exists() {
        bail!(
            "app not found at {} (build first?)",
            app_path.display()
        );
    }

    let bundle_identifier = settings
        .product_bundle_identifier
        .clone()
        .context("PRODUCT_BUNDLE_IDENTIFIER missing")?;

    Ok(LaunchArtifacts {
        app_path,
        bundle_identifier,
    })
}

fn extract_json_payload(output: &str) -> Option<String> {
    if serde_json::from_str::<serde_json::Value>(output).is_ok() {
        return Some(output.to_string());
    }
    let start = output.find('{').or_else(|| output.find('['))?;
    let end = output.rfind('}').or_else(|| output.rfind(']'))?;
    Some(output[start..=end].to_string())
}

#[derive(Debug, Deserialize)]
struct BuildSettingsEntry {
    #[serde(rename = "buildSettings")]
    build_settings: BuildSettingsMap,
}

#[derive(Debug, Deserialize)]
struct BuildSettingsMap {
    #[serde(rename = "TARGET_BUILD_DIR")]
    target_build_dir: Option<String>,
    #[serde(rename = "WRAPPER_NAME")]
    wrapper_name: Option<String>,
    #[serde(rename = "FULL_PRODUCT_NAME")]
    full_product_name: Option<String>,
    #[serde(rename = "PRODUCT_NAME")]
    product_name: Option<String>,
    #[serde(rename = "PRODUCT_BUNDLE_IDENTIFIER")]
    product_bundle_identifier: Option<String>,
}
