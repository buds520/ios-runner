use std::path::Path;

use anyhow::{Context, Result};
use serde_json::json;

use crate::bootstrap::zed_task_preamble;
use crate::detect::DetectedProject;

/// Run in Zed's terminal panel; auto-download CLI if needed (no cargo).
pub fn shell_task(label: &str, subcommand: &str) -> serde_json::Value {
    let preamble = zed_task_preamble();
    json!({
        "label": label,
        "command": "/bin/zsh",
        "args": [
            "-lc",
            format!(
                "{preamble}\nexport IOS_RUNNER_RAW_LOG=1\ncd \"$ZED_WORKTREE_ROOT\" && \"$ir_bin\" ensure && \"$ir_bin\" {subcommand}"
            )
        ],
        "allow_concurrent_runs": false,
        "reveal": "always",
        "hide": "never",
        "save": "all",
        "use_new_terminal": true
    })
}

pub fn write_zed_tasks(root: &Path, project: &DetectedProject) -> Result<()> {
    let zed_dir = root.join(".zed");
    std::fs::create_dir_all(&zed_dir).context("create .zed directory")?;

    let mut tasks = vec![
        shell_task("iOS-Runner: Run", "run"),
        shell_task("iOS-Runner: Select Scheme & Device", "configure --run"),
        shell_task("iOS-Runner: Select Only (no run)", "configure --no-run"),
        shell_task("iOS-Runner: Build", "build"),
        shell_task("iOS-Runner: Build (verbose)", "build --verbose"),
        shell_task("iOS-Runner: Resolve Swift Packages", "resolve-packages"),
    ];

    if project.has_podfile {
        let preamble = zed_task_preamble();
        tasks.push(json!({
            "label": "iOS-Runner: Pod Install",
            "command": "/bin/zsh",
            "args": ["-lc", format!("{preamble}\ncd \"$ZED_WORKTREE_ROOT\" && pod install")],
            "allow_concurrent_runs": false,
            "reveal": "always",
            "hide": "never",
            "save": "all",
            "use_new_terminal": true
        }));
    }

    let path = zed_dir.join("tasks.json");
    let text = serde_json::to_string_pretty(&tasks).context("serialize tasks.json")?;
    std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
