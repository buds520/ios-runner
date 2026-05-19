use std::path::Path;

use anyhow::{Context, Result};
use serde_json::json;

use crate::bootstrap::{lang_for_task_script, zed_task_preamble};
use crate::detect::DetectedProject;
use crate::locale::Lang;

fn zed_task_shell_fields(script: String) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert("command".into(), json!("/bin/zsh"));
    map.insert("args".into(), json!(["-lc", script]));
    map.insert("allow_concurrent_runs".into(), json!(false));
    map.insert("reveal".into(), json!("always"));
    map.insert("hide".into(), json!("never"));
    map.insert("save".into(), json!("all"));
    map.insert("use_new_terminal".into(), json!(true));
    // Zed 默认会打印「Task finished / Command: /bin/zsh -i -c …」，容易像报错
    map.insert("show_command".into(), json!(false));
    map.insert("show_summary".into(), json!(false));
    map
}

/// Run in Zed's terminal panel; auto-download CLI if needed (no cargo).
pub fn shell_task(label: &str, subcommand: &str) -> serde_json::Value {
    shell_task_with_lang(label, subcommand, Lang::ZhCn)
}

pub fn shell_task_with_lang(label: &str, subcommand: &str, lang: Lang) -> serde_json::Value {
    let preamble = zed_task_preamble(lang);
    let script = format!(
        "{preamble}\nexport IOS_RUNNER_RAW_LOG=1\ncd \"$ZED_WORKTREE_ROOT\" && \"$ir_bin\" ensure && \"$ir_bin\" {subcommand}"
    );
    let mut map = zed_task_shell_fields(script);
    map.insert("label".into(), json!(label));
    serde_json::Value::Object(map)
}

pub fn write_zed_tasks(root: &Path, project: &DetectedProject) -> Result<()> {
    let zed_dir = root.join(".zed");
    std::fs::create_dir_all(&zed_dir).context("create .zed directory")?;
    let lang = lang_for_task_script(Some(root));
    let st = |label, sub| shell_task_with_lang(label, sub, lang);

    let mut tasks = vec![
        st("iOS-Runner: Run", "run"),
        st("iOS-Runner: Select Scheme & Device", "configure --run"),
        st("iOS-Runner: Select Only (no run)", "configure --no-run"),
        st("iOS-Runner: Build", "build"),
        st("iOS-Runner: Build (verbose)", "build --verbose"),
        st("iOS-Runner: Resolve Swift Packages", "resolve-packages"),
    ];

    if project.has_podfile {
        let preamble = zed_task_preamble(lang);
        let script = format!("{preamble}\ncd \"$ZED_WORKTREE_ROOT\" && pod install");
        let mut map = zed_task_shell_fields(script);
        map.insert("label".into(), json!("iOS-Runner: Pod Install"));
        tasks.push(serde_json::Value::Object(map));
    }

    let path = zed_dir.join("tasks.json");
    let text = serde_json::to_string_pretty(&tasks).context("serialize tasks.json")?;
    std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
