use std::path::Path;

use anyhow::Result;

use crate::config::RunnerConfig;
use crate::detect::{create_config, detect_project};
use crate::destination::validate_xcodebuild_destination;
use crate::xcodebuild::default_simulator_destination;
use crate::global_store::{
    canonical_root, config_file_path, load_config, load_global_file, save_config,
    should_write_project_tasks,
};
use crate::tasks::write_zed_tasks;

/// Idempotent: detect Xcode project and save settings to global config (not in the project tree).
pub fn ensure_project(root: &Path) -> Result<EnsureReport> {
    let project = detect_project(root)?;
    let root_key = canonical_root(root).to_string_lossy().to_string();

    let mut wrote_config = false;
    let mut global_file = load_global_file()?;

    if !global_file.projects.contains_key(&root_key) {
        if let Ok(local) = RunnerConfig::load_local(root) {
            save_config(root, &local)?;
            wrote_config = true;
            global_file = load_global_file()?;
        }
    }

    if !global_file.projects.contains_key(&root_key) {
        let config = create_config(root, &project)?;
        config.save(root)?;
        wrote_config = true;
    }

    let mut wrote_tasks = false;
    if should_write_project_tasks() {
        if !root.join(".zed/tasks.json").is_file() {
            wrote_tasks = true;
        }
        write_zed_tasks(root, &project)?;
    }

    let mut config = load_config(root)?;
    if validate_xcodebuild_destination(&config.destination).is_err() {
        if let Ok(dest) = default_simulator_destination(root, &project, &config.scheme) {
            config.destination = dest;
            config.save(root)?;
            wrote_config = true;
        }
    }

    Ok(EnsureReport {
        scheme: config.scheme,
        path: config.path,
        destination: config.destination,
        wrote_config,
        wrote_tasks,
        has_podfile: project.has_podfile,
        global_config: config_file_path()?,
    })
}

#[derive(Debug, Clone)]
pub struct EnsureReport {
    pub scheme: String,
    pub path: String,
    pub destination: String,
    pub wrote_config: bool,
    pub wrote_tasks: bool,
    pub has_podfile: bool,
    pub global_config: std::path::PathBuf,
}
