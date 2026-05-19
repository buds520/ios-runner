use std::path::Path;

use anyhow::Result;

use crate::config::RunnerConfig;
use crate::detect::{create_config, detect_project};
use crate::tasks::write_zed_tasks;

/// Idempotent: detect Xcode project and write `.ios-runner.toml` + `.zed/tasks.json` if needed.
pub fn ensure_project(root: &Path) -> Result<EnsureReport> {
    let project = detect_project(root)?;
    let config_path = root.join(RunnerConfig::FILE_NAME);
    let tasks_path = root.join(".zed/tasks.json");

    let mut wrote_config = false;
    let mut wrote_tasks = false;

    let legacy_config = root.join(".xcode-pilot.toml");
    if !config_path.is_file() && !legacy_config.is_file() {
        let config = create_config(root, &project)?;
        config.save(root)?;
        wrote_config = true;
    }

    if !tasks_path.is_file() {
        wrote_tasks = true;
    }
    // Always refresh tasks so Zed picks up template changes (terminal, labels).
    write_zed_tasks(root, &project)?;

    let config = RunnerConfig::load(root)?;

    Ok(EnsureReport {
        scheme: config.scheme,
        path: config.path,
        destination: config.destination,
        wrote_config,
        wrote_tasks,
        has_podfile: project.has_podfile,
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
}
