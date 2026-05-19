use std::path::Path;

use anyhow::{Context, Result};
use serde_json::json;

use crate::detect::DetectedProject;

fn shell_task(label: &str, subcommand: &str) -> serde_json::Value {
    json!({
        "label": label,
        "command": "/bin/zsh",
        "args": [
            "-lc",
            format!(
                "export PATH=\"$HOME/.cargo/bin:$PATH\" && cd \"$ZED_WORKTREE_ROOT\" && ios-runner {subcommand}"
            )
        ],
        "allow_concurrent_runs": false,
        "reveal": "always",
        "hide": "never",
        "save": "all",
        "use_new_terminal": false
    })
}

pub fn write_zed_tasks(root: &Path, project: &DetectedProject) -> Result<()> {
    let zed_dir = root.join(".zed");
    std::fs::create_dir_all(&zed_dir).context("create .zed directory")?;

    let mut tasks = vec![
        json!({
            "label": "iOS-Runner: Configure",
            "command": "/bin/zsh",
            "args": [
                "-lc",
                "export PATH=\"$HOME/.cargo/bin:$PATH\" && cd \"$ZED_WORKTREE_ROOT\" && ios-runner configure"
            ],
            "allow_concurrent_runs": false,
            "reveal": "always",
            "hide": "never",
            "save": "all",
            "use_new_terminal": true
        }),
        shell_task("iOS-Runner: Build", "build"),
        shell_task("iOS-Runner: Run", "run"),
        shell_task("iOS-Runner: Resolve Swift Packages", "resolve-packages"),
    ];

    if project.has_podfile {
        tasks.push(json!({
            "label": "iOS-Runner: Pod Install",
            "command": "/bin/zsh",
            "args": ["-lc", "cd \"$ZED_WORKTREE_ROOT\" && pod install"],
            "allow_concurrent_runs": false,
            "reveal": "always",
            "hide": "never",
            "save": "all",
            "use_new_terminal": false
        }));
    }

    let path = zed_dir.join("tasks.json");
    let text = serde_json::to_string_pretty(&tasks).context("serialize tasks.json")?;
    std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
