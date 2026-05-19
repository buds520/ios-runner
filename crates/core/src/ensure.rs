use std::path::Path;

use anyhow::Result;

use crate::config::PilotConfig;
use crate::detect::{create_config, detect_project};
use crate::tasks::write_zed_tasks;

/// Idempotent: detect Xcode project and write `.xcode-pilot.toml` + `.zed/tasks.json` if needed.
pub fn ensure_project(root: &Path) -> Result<EnsureReport> {
    let project = detect_project(root)?;
    let config_path = root.join(PilotConfig::FILE_NAME);
    let tasks_path = root.join(".zed/tasks.json");

    let mut wrote_config = false;
    let mut wrote_tasks = false;

    if !config_path.is_file() {
        let config = create_config(root, &project)?;
        config.save(root)?;
        wrote_config = true;
    }

    if !tasks_path.is_file() {
        write_zed_tasks(root, &project)?;
        wrote_tasks = true;
    }

    let config = PilotConfig::load(root)?;

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
