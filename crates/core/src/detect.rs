use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;

use crate::config::{ProjectKind, RunnerConfig};
use crate::global_store::{global_derived_data_path, load_global_file};
use crate::xcodebuild::{default_simulator_destination, list_schemes};

#[derive(Debug, Clone)]
pub struct DetectedProject {
    pub kind: ProjectKind,
    pub path: PathBuf,
    pub has_podfile: bool,
    pub has_package_swift: bool,
}

pub fn detect_project(root: &Path) -> Result<DetectedProject> {
    let has_podfile = root.join("Podfile").is_file();
    let has_package_swift = root.join("Package.swift").is_file();

    if has_podfile {
        if let Some(ws) = find_workspace(root, true)? {
            return Ok(DetectedProject {
                kind: ProjectKind::Workspace,
                path: ws,
                has_podfile: true,
                has_package_swift,
            });
        }
        bail!(
            "Podfile found but no .xcworkspace. Run `pod install` to generate the workspace."
        );
    }

    if let Some(ws) = find_workspace(root, false)? {
        return Ok(DetectedProject {
            kind: ProjectKind::Workspace,
            path: ws,
            has_podfile: false,
            has_package_swift,
        });
    }

    if let Some(proj) = find_xcodeproj(root)? {
        return Ok(DetectedProject {
            kind: ProjectKind::Project,
            path: proj,
            has_podfile: false,
            has_package_swift,
        });
    }

    bail!(
        "no .xcworkspace or .xcodeproj found. Open the directory that contains your Xcode project."
    );
}

fn find_workspace(root: &Path, cocoapods: bool) -> Result<Option<PathBuf>> {
    let mut candidates = Vec::new();
    for entry in WalkDir::new(root).max_depth(3).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("xcworkspace") {
            continue;
        }
        if path.components().any(|c| c.as_os_str() == "Pods") {
            continue;
        }
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        // Embedded workspace inside .xcodeproj — use the .xcodeproj instead.
        if name == "project.xcworkspace" {
            continue;
        }
        candidates.push(path.to_path_buf());
    }
    candidates.sort_by_key(|p| p.file_name().map(|s| s.len()).unwrap_or(0));
    candidates.reverse();
    Ok(candidates.into_iter().next())
}

fn find_xcodeproj(root: &Path) -> Result<Option<PathBuf>> {
    let mut candidates = Vec::new();
    for entry in WalkDir::new(root).max_depth(3).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("xcodeproj") {
            continue;
        }
        if path.components().any(|c| c.as_os_str() == "Pods") {
            continue;
        }
        candidates.push(path.to_path_buf());
    }
    candidates.sort_by_key(|p| p.file_name().map(|s| s.len()).unwrap_or(0));
    candidates.reverse();
    Ok(candidates.into_iter().next())
}

pub fn create_config(root: &Path, project: &DetectedProject) -> Result<RunnerConfig> {
    let schemes = list_schemes(root, project)?;
    let scheme = pick_default_scheme(&schemes)
        .with_context(|| format!("no schemes in {}", project.path.display()))?
        .to_string();

    let destination = default_simulator_destination(root, project, &scheme)?;

    let rel = project
        .path
        .strip_prefix(root)
        .unwrap_or(&project.path)
        .to_string_lossy()
        .to_string();

    Ok(RunnerConfig {
        kind: project.kind,
        path: rel,
        scheme,
        configuration: "Debug".to_string(),
        destination,
        derived_data: global_derived_data_path(root)?
            .to_string_lossy()
            .to_string(),
        xcbeautify: load_global_file().map(|f| f.defaults.xcbeautify).unwrap_or(true),
        resolve_packages_before_build: load_global_file()
            .map(|f| f.defaults.resolve_packages_before_build)
            .unwrap_or(true),
        bring_simulator_to_foreground: load_global_file()
            .map(|f| f.defaults.bring_simulator_to_foreground)
            .unwrap_or(true),
        development_team: None,
        language: std::env::var("IOS_RUNNER_LANG").unwrap_or_else(|_| {
            load_global_file()
                .map(|f| f.defaults.language)
                .unwrap_or_else(|_| "zh-CN".to_string())
        }),
    })
}

fn pick_default_scheme<'a>(schemes: &'a [String]) -> Option<&'a str> {
    schemes
        .iter()
        .find(|s| !s.starts_with("Pods-"))
        .or_else(|| schemes.first())
        .map(|s| s.as_str())
}
