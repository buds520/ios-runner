use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::tasks::shell_task;

const TASK_LABEL_PREFIX: &str = "iOS-Runner:";

/// Default Zed tasks (work in any iOS project via `$ZED_WORKTREE_ROOT`).
pub fn default_task_list() -> Vec<serde_json::Value> {
    vec![
        shell_task("iOS-Runner: Setup Project", "ensure"),
        shell_task("iOS-Runner: Run", "run"),
        shell_task("iOS-Runner: Select Scheme & Device", "configure --run"),
        shell_task("iOS-Runner: Build", "build"),
    ]
}

pub fn zed_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("home directory")?;
    Ok(home.join(".config/zed"))
}

/// Write global Zed tasks, replacing any previous iOS-Runner entries (fixes stale task scripts).
pub fn install_global_zed_tasks() -> Result<PathBuf> {
    let dir = zed_config_dir()?;
    std::fs::create_dir_all(&dir).context("create ~/.config/zed")?;
    let path = dir.join("tasks.json");

    let mut tasks: Vec<Value> = if path.is_file() {
        let text = std::fs::read_to_string(&path).context("read global tasks.json")?;
        serde_json::from_str(&text).unwrap_or_default()
    } else {
        Vec::new()
    };

    tasks.retain(|t| {
        t.get("label")
            .and_then(|l| l.as_str())
            .is_none_or(|label| !label.starts_with(TASK_LABEL_PREFIX))
    });

    for task in default_task_list() {
        tasks.push(task);
    }

    let text = serde_json::to_string_pretty(&tasks).context("serialize tasks")?;
    std::fs::write(&path, text).context("write global tasks.json")?;
    Ok(path)
}
