use std::path::Path;

use anyhow::{Context, Result};
use serde_json::json;

use crate::bootstrap::{lang_for_task_script, zed_task_preamble, CLI_PATH_SHELL};
use crate::detect::DetectedProject;
use crate::locale::Lang;

/// Prefix for all Zed task labels (`iOS-Runner: …`).
pub const TASK_LABEL_PREFIX: &str = "iOS-Runner:";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskInstallTarget {
    Global,
    Project,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskDef {
    pub subcommand: &'static str,
    pub target: TaskInstallTarget,
    pub label_zh: &'static str,
    pub label_en: &'static str,
}

/// Single source of truth for Zed task labels and CLI subcommands.
pub const TASK_DEFS: &[TaskDef] = &[
    TaskDef {
        subcommand: "doctor",
        target: TaskInstallTarget::Global,
        label_zh: "检查环境",
        label_en: "Doctor",
    },
    TaskDef {
        subcommand: "ensure",
        target: TaskInstallTarget::Global,
        label_zh: "初始化项目",
        label_en: "Initialize Project",
    },
    TaskDef {
        subcommand: "run",
        target: TaskInstallTarget::Global,
        label_zh: "运行",
        label_en: "Run",
    },
    TaskDef {
        subcommand: "configure --no-run",
        target: TaskInstallTarget::Global,
        label_zh: "选择 Scheme 与运行目标",
        label_en: "Select Scheme & Destination",
    },
    TaskDef {
        subcommand: "build",
        target: TaskInstallTarget::Global,
        label_zh: "编译",
        label_en: "Build",
    },
    // Project-only extras (Run/Build/Configure live in global ~/.config/zed/tasks.json)
    TaskDef {
        subcommand: "configure --no-run",
        target: TaskInstallTarget::Project,
        label_zh: "仅选择（不运行）",
        label_en: "Select Only (no run)",
    },
    TaskDef {
        subcommand: "build --verbose",
        target: TaskInstallTarget::Project,
        label_zh: "编译（详细日志）",
        label_en: "Build (verbose)",
    },
    TaskDef {
        subcommand: "resolve-packages",
        target: TaskInstallTarget::Project,
        label_zh: "解析 Swift Packages",
        label_en: "Resolve Swift Packages",
    },
];

pub fn task_label(suffix: &str) -> String {
    format!("{TASK_LABEL_PREFIX} {suffix}")
}

pub fn task_label_for_def(def: &TaskDef, lang: Lang) -> String {
    let suffix = match lang {
        Lang::ZhCn => def.label_zh,
        Lang::En => def.label_en,
    };
    task_label(suffix)
}

/// Label for a global task used in `~/.config/zed/keymap.json` (`task::Spawn`).
pub fn global_keymap_task_label(subcommand: &'static str, lang: Lang) -> Option<String> {
    TASK_DEFS
        .iter()
        .find(|d| d.target == TaskInstallTarget::Global && d.subcommand == subcommand)
        .map(|d| task_label_for_def(d, lang))
}

/// True when `tasks.json` contains a current iOS-Runner task set (any supported label).
pub fn tasks_json_has_ios_runner_tasks(text: &str) -> bool {
    if text.contains("curl -fsSL") || text.contains("$ir_bin") {
        return false;
    }
    text.contains("iOS-Runner: Doctor")
        || text.contains("iOS-Runner: 检查环境")
        || text.contains("iOS-Runner: Run")
        || text.contains("iOS-Runner: 运行")
        || text.contains("iOS-Runner: Initialize Project")
        || text.contains("iOS-Runner: 初始化项目")
        || text.contains("iOS-Runner: Setup Project")
}

pub fn shell_tasks_for_target(target: TaskInstallTarget, lang: Lang) -> Vec<serde_json::Value> {
    TASK_DEFS
        .iter()
        .filter(|d| d.target == target)
        .map(|d| shell_task_with_lang(&task_label_for_def(d, lang), d.subcommand, lang))
        .collect()
}

/// Whether `.zed/tasks.json` should be rewritten (missing, forced, or legacy duplicates).
pub fn should_refresh_project_tasks(path: &Path) -> bool {
    if !path.is_file() {
        return true;
    }
    if crate::global_store::should_write_project_tasks() {
        return true;
    }
    std::fs::read_to_string(path)
        .map(|t| project_tasks_file_has_global_duplicates(&t))
        .unwrap_or(false)
}

/// Labels that belong in global tasks only — if present in `.zed/tasks.json`, refresh project file.
pub fn project_tasks_file_has_global_duplicates(text: &str) -> bool {
    text.contains("\"label\": \"iOS-Runner: 运行\"")
        || text.contains("\"label\": \"iOS-Runner: Run\"")
        || text.contains("\"label\": \"iOS-Runner: 检查环境\"")
        || text.contains("\"label\": \"iOS-Runner: Doctor\"")
        || text.contains("\"label\": \"iOS-Runner: 编译\"")
        || text.contains("\"label\": \"iOS-Runner: Build\"")
        || text.contains("\"label\": \"iOS-Runner: 初始化项目\"")
        || text.contains("\"label\": \"iOS-Runner: 选择 Scheme 与设备\"")
        || text.contains("\"label\": \"iOS-Runner: 选择 Scheme 与运行目标\"")
        || text.contains("\"label\": \"iOS-Runner: Select Scheme & Destination\"")
}

fn zed_task_shell_fields(script: String) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    map.insert("command".into(), json!("/bin/zsh"));
    map.insert("args".into(), json!(["-fc", script]));
    map.insert("allow_concurrent_runs".into(), json!(false));
    map.insert("reveal".into(), json!("always"));
    map.insert("hide".into(), json!("never"));
    map.insert("save".into(), json!("all"));
    map.insert("use_new_terminal".into(), json!(false));
    // 非 login shell，减少 Zed 拉取工程环境时在面板里刷一整屏 KEY=value
    map.insert(
        "shell".into(),
        json!({
            "with_arguments": {
                "program": "/bin/zsh",
                "args": ["-f"]
            }
        }),
    );
    // Zed 默认会打印「Task finished / Command: /bin/zsh -i -c …」，容易像报错
    map.insert("show_command".into(), json!(false));
    map.insert("show_summary".into(), json!(false));
    map
}

fn task_body(subcommand: &str) -> String {
    if subcommand == "ensure" {
        format!("{CLI_PATH_SHELL} ensure")
    } else {
        format!("{CLI_PATH_SHELL} ensure --quiet && {CLI_PATH_SHELL} {subcommand}")
    }
}

/// Run in Zed's terminal panel.
#[allow(dead_code)]
pub fn shell_task(label: &str, subcommand: &str) -> serde_json::Value {
    shell_task_with_lang(label, subcommand, Lang::ZhCn)
}

pub fn shell_task_with_lang(label: &str, subcommand: &str, lang: Lang) -> serde_json::Value {
    let preamble = zed_task_preamble(lang);
    let script = format!(
        "{preamble}\ncd \"${{ZED_WORKTREE_ROOT:.}}\" && {}",
        task_body(subcommand)
    );
    let mut map = zed_task_shell_fields(script);
    map.insert("label".into(), json!(label));
    serde_json::Value::Object(map)
}

pub fn write_zed_tasks(root: &Path, project: &DetectedProject) -> Result<()> {
    let zed_dir = root.join(".zed");
    std::fs::create_dir_all(&zed_dir).context("create .zed directory")?;
    let lang = lang_for_task_script(Some(root));
    let mut tasks = shell_tasks_for_target(TaskInstallTarget::Project, lang);

    if project.has_podfile {
        let preamble = zed_task_preamble(lang);
        let script = format!("{preamble}\ncd \"${{ZED_WORKTREE_ROOT:.}}\" && pod install");
        let mut map = zed_task_shell_fields(script);
        map.insert("label".into(), json!("iOS-Runner: Pod Install"));
        tasks.push(serde_json::Value::Object(map));
    }

    let path = zed_dir.join("tasks.json");
    let text = serde_json::to_string_pretty(&tasks).context("serialize tasks.json")?;
    std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_task_label_zh() {
        let def = TASK_DEFS
            .iter()
            .find(|d| d.subcommand == "ensure" && d.target == TaskInstallTarget::Global)
            .expect("ensure task");
        assert_eq!(
            task_label_for_def(def, Lang::ZhCn),
            "iOS-Runner: 初始化项目"
        );
    }

    #[test]
    fn detects_localized_tasks_json() {
        assert!(tasks_json_has_ios_runner_tasks(
            r#"[{"label":"iOS-Runner: 初始化项目"}]"#
        ));
    }

    #[test]
    fn project_tasks_exclude_global_duplicates() {
        let project = shell_tasks_for_target(TaskInstallTarget::Project, Lang::ZhCn);
        let labels: Vec<_> = project
            .iter()
            .filter_map(|t| t.get("label").and_then(|l| l.as_str()))
            .collect();
        assert!(!labels.contains(&"iOS-Runner: 运行"));
        assert!(!labels.contains(&"iOS-Runner: 检查环境"));
        assert!(labels.iter().any(|l| l.contains("详细日志")));
    }

    #[test]
    fn global_tasks_include_doctor() {
        let global = shell_tasks_for_target(TaskInstallTarget::Global, Lang::En);
        let labels: Vec<_> = global
            .iter()
            .filter_map(|t| t.get("label").and_then(|l| l.as_str()))
            .collect();
        assert!(labels.contains(&"iOS-Runner: Doctor"));
    }

    #[test]
    fn configure_task_uses_destination_language() {
        let def = TASK_DEFS
            .iter()
            .find(|d| d.subcommand == "configure --no-run" && d.target == TaskInstallTarget::Global)
            .expect("configure task");
        assert_eq!(
            task_label_for_def(def, Lang::En),
            "iOS-Runner: Select Scheme & Destination"
        );
        assert_eq!(
            task_label_for_def(def, Lang::ZhCn),
            "iOS-Runner: 选择 Scheme 与运行目标"
        );
    }
}
