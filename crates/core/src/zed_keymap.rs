use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_json::{Value, json};

/// Recommended one-key actions (no `task: spawn` picker).
pub fn default_ios_runner_bindings() -> serde_json::Map<String, Value> {
    let mut bindings = serde_json::Map::new();
    let spawn = |name: &str| json!(["task::Spawn", { "task_name": name }]);

    bindings.insert("cmd-shift-r".into(), spawn("iOS-Runner: Run"));
    bindings.insert("cmd-shift-b".into(), spawn("iOS-Runner: Build"));
    bindings.insert("cmd-shift-i".into(), spawn("iOS-Runner: Select Scheme & Device"));
    bindings.insert("cmd-shift-e".into(), spawn("iOS-Runner: Setup Project"));
    bindings
}

/// Keys written by `install_global_zed_keymap`.
pub fn ios_runner_binding_keys() -> &'static [&'static str] {
    &["cmd-shift-r", "cmd-shift-b", "cmd-shift-i", "cmd-shift-e"]
}

fn binding_targets_ios_runner(value: &Value) -> bool {
    let Some(arr) = value.as_array() else {
        return false;
    };
    if arr.first().and_then(|v| v.as_str()) != Some("task::Spawn") {
        return false;
    }
    arr.get(1)
        .and_then(|v| v.get("task_name"))
        .and_then(|v| v.as_str())
        .is_some_and(|name| name.starts_with("iOS-Runner:"))
}

/// Remove iOS-Runner keybindings from `~/.config/zed/keymap.json`.
pub fn uninstall_global_zed_keymap() -> Result<Option<PathBuf>> {
    let dir = zed_config_dir()?;
    let path = dir.join("keymap.json");
    if !path.is_file() {
        return Ok(None);
    }

    let text = std::fs::read_to_string(&path).context("read keymap.json")?;
    let mut entries: Vec<Value> = serde_json::from_str(&text).unwrap_or_default();
    let mut changed = false;

    for entry in &mut entries {
        let Some(bindings) = entry
            .as_object_mut()
            .and_then(|o| o.get_mut("bindings"))
            .and_then(|b| b.as_object_mut())
        else {
            continue;
        };
        let keys: Vec<String> = bindings.keys().cloned().collect();
        for key in keys {
            let remove = bindings.get(&key).is_some_and(binding_targets_ios_runner)
                || ios_runner_binding_keys().contains(&key.as_str());
            if remove {
                bindings.remove(&key);
                changed = true;
            }
        }
    }

    if !changed {
        return Ok(None);
    }

    let text = serde_json::to_string_pretty(&entries).context("serialize keymap")?;
    std::fs::write(&path, text).context("write keymap.json")?;
    Ok(Some(path))
}

pub fn zed_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("home directory")?;
    Ok(home.join(".config/zed"))
}

/// Merge iOS-Runner bindings into `~/.config/zed/keymap.json`.
pub fn install_global_zed_keymap() -> Result<PathBuf> {
    let dir = zed_config_dir()?;
    std::fs::create_dir_all(&dir).context("create ~/.config/zed")?;
    let path = dir.join("keymap.json");

    let mut entries: Vec<Value> = if path.is_file() {
        let text = std::fs::read_to_string(&path).context("read keymap.json")?;
        serde_json::from_str(&text).unwrap_or_else(|_| vec![json!({ "bindings": {} })])
    } else {
        vec![]
    };

    let ours = default_ios_runner_bindings();
    if let Some(workspace) = entries.iter_mut().find(|e| e.get("context") == Some(&json!("Workspace"))) {
        let bindings = workspace
            .as_object_mut()
            .and_then(|o| o.get_mut("bindings"))
            .and_then(|b| b.as_object_mut());
        if let Some(map) = bindings {
            for (k, v) in ours {
                map.entry(k).or_insert(v);
            }
        } else {
            workspace
                .as_object_mut()
                .expect("workspace entry")
                .insert("bindings".into(), Value::Object(ours));
        }
    } else {
        entries.push(json!({
            "context": "Workspace",
            "bindings": Value::Object(ours),
        }));
    }

    let text = serde_json::to_string_pretty(&entries).context("serialize keymap")?;
    std::fs::write(&path, text).context("write keymap.json")?;
    Ok(path)
}
