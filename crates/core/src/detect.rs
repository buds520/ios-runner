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

fn find_xcode_file(
    root: &Path,
    extension: &str,
    extra_filter: impl Fn(&Path) -> bool,
) -> Result<Option<PathBuf>> {
    let mut candidates: Vec<PathBuf> = WalkDir::new(root)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some(extension))
        .filter(|path| !path.components().any(|c| c.as_os_str() == "Pods"))
        .filter(|path| extra_filter(path))
        .collect();
    candidates.sort_by_key(|p| p.file_name().map(|s| s.len()).unwrap_or(0));
    candidates.reverse();
    Ok(candidates.into_iter().next())
}

fn find_workspace(root: &Path, _cocoapods: bool) -> Result<Option<PathBuf>> {
    find_xcode_file(root, "xcworkspace", |path| {
        path.file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|name| name != "project.xcworkspace")
    })
}

fn find_xcodeproj(root: &Path) -> Result<Option<PathBuf>> {
    find_xcode_file(root, "xcodeproj", |_| true)
}

/// Schemes tied to this workspace/project name (drops unrelated Pods deps like AFNetworking).
pub fn filter_schemes_for_project(schemes: &[String], project: &DetectedProject) -> Vec<String> {
    let stem = project
        .path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let related: Vec<String> = schemes
        .iter()
        .filter(|s| !s.starts_with("Pods-"))
        .filter(|s| *s == stem || s.starts_with(stem))
        .cloned()
        .collect();
    if related.is_empty() {
        schemes
            .iter()
            .filter(|s| !s.starts_with("Pods-"))
            .cloned()
            .collect()
    } else {
        related
    }
}

pub fn pick_default_scheme(schemes: &[String], project: &DetectedProject) -> Option<String> {
    let filtered = filter_schemes_for_project(schemes, project);
    let stem = project.path.file_stem()?.to_str()?;
    if filtered.iter().any(|s| s == stem) {
        return Some(stem.to_string());
    }
    filtered
        .into_iter()
        .find(|s| !s.ends_with("Tests") && !s.ends_with("UITests"))
        .or_else(|| filter_schemes_for_project(schemes, project).into_iter().next())
}

pub fn create_config(root: &Path, project: &DetectedProject) -> Result<RunnerConfig> {
    let schemes = list_schemes(root, project)?;
    let scheme = pick_default_scheme(&schemes, project)
        .with_context(|| format!("no schemes in {}", project.path.display()))?;

    let destination = default_preferred_destination(root, project, &scheme)?;

    let rel = project
        .path
        .strip_prefix(root)
        .unwrap_or(&project.path)
        .to_string_lossy()
        .to_string();

    let defaults = load_global_file()
        .map(|f| f.defaults)
        .unwrap_or_default();

    Ok(RunnerConfig {
        kind: project.kind,
        path: rel,
        scheme,
        configuration: "Debug".to_string(),
        destination,
        derived_data: global_derived_data_path(root)?
            .to_string_lossy()
            .to_string(),
        xcbeautify: defaults.xcbeautify,
        resolve_packages_before_build: defaults.resolve_packages_before_build,
        bring_simulator_to_foreground: defaults.bring_simulator_to_foreground,
        development_team: None,
        language: std::env::var("IOS_RUNNER_LANG").unwrap_or(defaults.language),
    })
}

/// Prefer connected device, else iPhone simulator.
pub fn default_preferred_destination(
    root: &Path,
    project: &DetectedProject,
    scheme: &str,
) -> Result<String> {
    let destinations = crate::destination::list_run_destinations(root, project, scheme)?;
    if let Some(d) = destinations
        .iter()
        .find(|d| d.kind == crate::destination::DestinationKind::Device)
    {
        return Ok(d.destination.clone());
    }
    default_simulator_destination(root, project, scheme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_detect_root(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "ios-runner-detect-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn find_xcodeproj_ignores_pods_and_embedded_workspace() {
        let root = temp_detect_root("proj");
        let proj = root.join("App.xcodeproj");
        fs::create_dir_all(proj.join("project.xcworkspace")).unwrap();
        fs::write(proj.join("contents.xcworkspacedata"), "").ok();

        assert!(find_workspace(&root, false).unwrap().is_none());
        let found = find_xcodeproj(&root).unwrap().expect("xcodeproj");
        assert!(found.ends_with("App.xcodeproj"));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn find_workspace_skips_project_xcworkspace() {
        let root = temp_detect_root("ws");
        let ws = root.join("App.xcworkspace");
        fs::create_dir_all(&ws).unwrap();
        fs::write(ws.join("contents.xcworkspacedata"), "").ok();

        let found = find_workspace(&root, false).unwrap().expect("workspace");
        assert!(found.ends_with("App.xcworkspace"));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn filter_schemes_keeps_project_prefix() {
        let root = temp_detect_root("scheme-filter");
        let ws = root.join("Earphones.xcworkspace");
        fs::create_dir_all(&ws).unwrap();
        let project = DetectedProject {
            kind: ProjectKind::Workspace,
            path: ws,
            has_podfile: false,
            has_package_swift: false,
        };
        let schemes = vec![
            "AFNetworking".into(),
            "Earphones".into(),
            "EarphonesWidgetExtension".into(),
        ];
        let filtered = filter_schemes_for_project(&schemes, &project);
        assert!(filtered.contains(&"Earphones".to_string()));
        assert!(filtered.contains(&"EarphonesWidgetExtension".to_string()));
        assert!(!filtered.contains(&"AFNetworking".to_string()));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn pick_default_scheme_prefers_workspace_name() {
        let root = temp_detect_root("scheme-default");
        let ws = root.join("App.xcworkspace");
        fs::create_dir_all(&ws).unwrap();
        let project = DetectedProject {
            kind: ProjectKind::Workspace,
            path: ws,
            has_podfile: false,
            has_package_swift: false,
        };
        let schemes = vec!["AppTests".into(), "App".into(), "Pods-App".into()];
        assert_eq!(
            pick_default_scheme(&schemes, &project).as_deref(),
            Some("App")
        );
        let _ = fs::remove_dir_all(&root);
    }
}
