use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

use crate::config::ProjectKind;
use crate::detect::DetectedProject;
use crate::xcodebuild::add_project_args;

pub fn supported_platforms_for_scheme(
    root: &Path,
    project: &DetectedProject,
    scheme: &str,
) -> Result<Vec<String>> {
    let mut cmd = Command::new("xcodebuild");
    add_project_args(&mut cmd, project);
    cmd.args([
        "-scheme",
        scheme,
        "-configuration",
        "Debug",
        "-showBuildSettings",
    ]);

    let output = cmd
        .current_dir(root)
        .output()
        .context("xcodebuild -showBuildSettings")?;

    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let mut platforms = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("SUPPORTED_PLATFORMS = ") else {
            continue;
        };
        for token in rest.split_whitespace() {
            if !platforms.iter().any(|p| p == token) {
                platforms.push(token.to_string());
            }
        }
    }
    Ok(platforms)
}

pub fn platforms_support_ios(platforms: &[String]) -> bool {
    platforms.iter().any(|p| {
        p == "iphoneos"
            || p == "iphonesimulator"
            || p.starts_with("iphoneos")
            || p.starts_with("iphonesimulator")
    })
}

pub fn platforms_macos_only(platforms: &[String]) -> bool {
    !platforms.is_empty() && platforms.iter().all(|p| p == "macosx" || p == "macos")
}

pub fn scheme_is_macos_only(root: &Path, project: &DetectedProject, scheme: &str) -> Result<bool> {
    let platforms = supported_platforms_for_scheme(root, project, scheme)?;
    if !platforms.is_empty() {
        return Ok(platforms_macos_only(&platforms));
    }
    pbxproj_looks_macos_only(project)
}

fn pbxproj_looks_macos_only(project: &DetectedProject) -> Result<bool> {
    let pbx = match project.kind {
        ProjectKind::Project => project.path.join("project.pbxproj"),
        ProjectKind::Workspace => {
            let stem = project
                .path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            project
                .path
                .parent()
                .map(|p| p.join(format!("{stem}.xcodeproj/project.pbxproj")))
                .filter(|p| p.is_file())
                .unwrap_or_else(|| project.path.join("project.pbxproj"))
        }
    };
    if !pbx.is_file() {
        return Ok(false);
    }
    let text = std::fs::read_to_string(&pbx).with_context(|| format!("read {}", pbx.display()))?;
    let has_ios = text.contains("iphoneos")
        || text.contains("iphonesimulator")
        || text.contains("TARGETED_DEVICE_FAMILY");
    let has_mac =
        text.contains("SDKROOT = macosx") || text.contains("SUPPORTED_PLATFORMS = macosx");
    Ok(has_mac && !has_ios)
}
