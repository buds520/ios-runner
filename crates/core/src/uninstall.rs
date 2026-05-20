use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::bootstrap::INSTALL_DIR;
use crate::global_store::config_dir;
use crate::global_tasks::uninstall_global_zed_tasks;
use crate::zed_keymap::uninstall_global_zed_keymap;

#[derive(Debug, Clone, Default)]
pub struct UninstallOptions {
    /// Keep `~/.config/ios-runner/` (project run settings).
    pub keep_config: bool,
    /// Also delete `~/.ios-runner/DerivedData/` (build cache).
    pub purge_derived_data: bool,
}

#[derive(Debug, Clone, Default)]
pub struct UninstallReport {
    pub removed: Vec<String>,
    pub skipped: Vec<String>,
}

/// Remove iOS-Runner CLI, Zed tasks/keymap, and optional config/cache.
pub fn uninstall_ios_runner(options: &UninstallOptions) -> Result<UninstallReport> {
    let home = dirs::home_dir().context("home directory")?;
    let mut report = UninstallReport::default();

    remove_file_if_exists(&home.join(INSTALL_DIR).join("ios-runner"), &mut report)?;

    if let Ok(entries) = fs::read_dir(home.join(".ios-runner")) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(".bootstrap-") {
                remove_path(&entry.path(), &mut report)?;
            }
        }
    }

    if options.purge_derived_data {
        remove_path(&home.join(".ios-runner/DerivedData"), &mut report)?;
    } else {
        let derived = home.join(".ios-runner/DerivedData");
        if derived.exists() {
            report.skipped.push(derived.display().to_string());
        }
    }

    if !options.keep_config {
        remove_path(&config_dir()?, &mut report)?;
    } else if config_dir().map(|p| p.exists()).unwrap_or(false) {
        report.skipped.push(config_dir()?.display().to_string());
    }

    match uninstall_global_zed_tasks()? {
        Some(p) => report.removed.push(p.display().to_string()),
        None => report
            .skipped
            .push("~/.config/zed/tasks.json (no iOS-Runner tasks)".into()),
    }

    match uninstall_global_zed_keymap()? {
        Some(p) => report.removed.push(p.display().to_string()),
        None => report
            .skipped
            .push("~/.config/zed/keymap.json (no iOS-Runner bindings)".into()),
    }

    // Remove empty ~/.ios-runner if only markers/bin were there
    prune_empty_dir(&home.join(".ios-runner"), &mut report);

    Ok(report)
}

fn remove_file_if_exists(path: &Path, report: &mut UninstallReport) -> Result<()> {
    if path.is_file() {
        fs::remove_file(path).with_context(|| format!("remove {}", path.display()))?;
        report.removed.push(path.display().to_string());
    }
    Ok(())
}

fn remove_path(path: &Path, report: &mut UninstallReport) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if path.is_dir() {
        fs::remove_dir_all(path).with_context(|| format!("remove {}", path.display()))?;
    } else {
        fs::remove_file(path).with_context(|| format!("remove {}", path.display()))?;
    }
    report.removed.push(path.display().to_string());
    Ok(())
}

fn prune_empty_dir(path: &Path, report: &mut UninstallReport) {
    if !path.is_dir() {
        return;
    }
    if fs::read_dir(path)
        .map(|mut d| d.next().is_none())
        .unwrap_or(false)
        && fs::remove_dir(path).is_ok()
    {
        report.removed.push(path.display().to_string());
    }
}
