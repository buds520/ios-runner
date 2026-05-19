use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::bootstrap::lang_for_task_script;
use crate::tasks::shell_task_with_lang;

const TASK_LABEL_PREFIX: &str = "iOS-Runner:";

/// Detect legacy task scripts that curl-download CLI (superseded by extension bundle install).
fn task_uses_legacy_download(task: &Value) -> bool {
    let script = task
        .get("args")
        .and_then(|a| a.as_array())
        .and_then(|args| args.get(1))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    script.contains("正在下载命令行工具")
        || script.contains("Downloading CLI")
        || script.contains("curl -fsSL")
        || script.contains("$ir_bin")
        || script.contains("$IOS_RUNNER")
        || script.contains("if [ ! -x")
        || script.contains("${HOME}/.ios-runner")
        || script.contains("~/.ios-runner/bin/ios-runner")
}

/// Default Zed tasks (work in any iOS project via `$ZED_WORKTREE_ROOT`).
pub fn default_task_list() -> Vec<serde_json::Value> {
    let lang = lang_for_task_script(None);
    let st = |label, sub| shell_task_with_lang(label, sub, lang);
    vec![
        st("iOS-Runner: Setup Project", "ensure"),
        st("iOS-Runner: Run", "run"),
        st("iOS-Runner: Select Scheme & Device", "configure --run"),
        st("iOS-Runner: Build", "build"),
    ]
}

pub fn zed_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("home directory")?;
    Ok(home.join(".config/zed"))
}

fn should_remove_task(task: &Value) -> bool {
    let label = task.get("label").and_then(|l| l.as_str());
    if label.is_some_and(|l| l.starts_with(TASK_LABEL_PREFIX)) {
        return true;
    }
    task_uses_legacy_download(task)
}

/// Remove iOS-Runner tasks from `~/.config/zed/tasks.json`.
pub fn uninstall_global_zed_tasks() -> Result<Option<PathBuf>> {
    let dir = zed_config_dir()?;
    let path = dir.join("tasks.json");
    if !path.is_file() {
        return Ok(None);
    }

    let text = std::fs::read_to_string(&path).context("read global tasks.json")?;
    let mut tasks: Vec<Value> = serde_json::from_str(&text).unwrap_or_default();
    let before = tasks.len();
    tasks.retain(|t| !should_remove_task(t));
    if tasks.len() == before {
        return Ok(None);
    }

    if tasks.is_empty() {
        std::fs::remove_file(&path).context("remove empty tasks.json")?;
    } else {
        let text = serde_json::to_string_pretty(&tasks).context("serialize tasks")?;
        std::fs::write(&path, text).context("write global tasks.json")?;
    }
    Ok(Some(path))
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

    tasks.retain(|t| !should_remove_task(t));

    for task in default_task_list() {
        tasks.push(task);
    }

    let text = serde_json::to_string_pretty(&tasks).context("serialize tasks")?;
    std::fs::write(&path, text).context("write global tasks.json")?;
    Ok(path)
}
