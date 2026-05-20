use std::path::Path;

use anyhow::Result;

use crate::config::RunnerConfig;
use crate::destination::{DestinationKind, list_run_destinations, validate_xcodebuild_destination};
use crate::detect::{
    create_config, detect_project, filter_schemes_for_project, pick_default_scheme,
};
use crate::global_store::{
    config_file_path, config_lookup_keys, load_config_for_project, load_global_file,
    save_config_for_project,
};
use crate::locale::t;
use crate::prompt::{is_interactive_tty, pick_one_with_default};
use crate::tasks::write_zed_tasks;
use crate::terminal_ui::{print_project_context, section};
use crate::xcodebuild::{default_simulator_destination, list_schemes};

/// Idempotent: detect Xcode project and save settings to global config.
pub fn ensure_project(root: &Path) -> Result<EnsureReport> {
    let project = detect_project(root)?;
    let keys = config_lookup_keys(root, &project);
    let mut wrote_config = false;
    let mut global_file = load_global_file()?;

    let has_config = keys.iter().any(|k| global_file.projects.contains_key(k));
    if !has_config {
        if let Ok(local) = RunnerConfig::load_local(root) {
            save_config_for_project(root, &project, &local)?;
            wrote_config = true;
            global_file = load_global_file()?;
        }
    }

    if !keys.iter().any(|k| global_file.projects.contains_key(k)) {
        if is_interactive_tty() {
            let config = interactive_configure(root, &project)?;
            save_config_for_project(root, &project, &config)?;
            wrote_config = true;
        } else {
            let config = create_config(root, &project)?;
            save_config_for_project(root, &project, &config)?;
            wrote_config = true;
        }
    }

    let tasks_root = project_tasks_root(root, &project);
    let mut wrote_tasks = false;
    let project_tasks = tasks_root.join(".zed/tasks.json");
    let needs_project_tasks = crate::tasks::should_refresh_project_tasks(&project_tasks);
    if needs_project_tasks {
        if !project_tasks.is_file() {
            wrote_tasks = true;
        }
        write_zed_tasks(&tasks_root, &project)?;
    }

    let mut config = load_config_for_project(root, &project)?;
    if validate_xcodebuild_destination(&config.destination).is_err() {
        if let Ok(dest) = default_simulator_destination(root, &project, &config.scheme) {
            config.destination = dest;
            save_config_for_project(root, &project, &config)?;
            wrote_config = true;
        }
    }

    if wrote_config && is_interactive_tty() {
        print_ensure_summary(&config);
    }

    Ok(EnsureReport {
        scheme: config.scheme,
        path: config.path,
        destination: config.destination,
        wrote_config,
        wrote_tasks,
        has_podfile: project.has_podfile,
        global_config: config_file_path()?,
    })
}

fn project_tasks_root(root: &Path, project: &crate::detect::DetectedProject) -> std::path::PathBuf {
    project
        .path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root.to_path_buf())
}

fn interactive_configure(root: &Path, project: &crate::detect::DetectedProject) -> Result<RunnerConfig> {
    section(
        t("首次配置 iOS-Runner", "First-time iOS-Runner setup"),
        Some(&project.path.display().to_string()),
    );

    let all_schemes = list_schemes(root, project)?;
    let scheme_options = filter_schemes_for_project(&all_schemes, project);
    let default_scheme = pick_default_scheme(&all_schemes, project)
        .unwrap_or_else(|| scheme_options.first().cloned().unwrap_or_default());
    let default_scheme_idx = scheme_options
        .iter()
        .position(|s| s == &default_scheme)
        .unwrap_or(0);

    let scheme_idx = pick_one_with_default(
        t("选择 Scheme", "Select scheme"),
        &scheme_options,
        default_scheme_idx,
    )?;
    let scheme = scheme_options[scheme_idx].clone();

    let destinations = list_run_destinations(root, project, &scheme)?;
    if destinations.is_empty() {
        anyhow::bail!(
            "{}",
            t(
                "未找到模拟器或真机。请在 Xcode → Settings → Platforms 安装 iOS 模拟器，或连接真机并在 Xcode 中完成信任与签名。",
                "No simulator or device found. Install an iOS simulator (Xcode → Settings → Platforms) or connect a device and set up signing in Xcode.",
            )
        );
    }
    let dest_labels: Vec<String> = destinations.iter().map(|d| d.menu_label()).collect();
    let default_dest_idx = destinations
        .iter()
        .position(|d| d.kind == DestinationKind::Device)
        .unwrap_or(0);

    let dest_idx = pick_one_with_default(
        t(
            "选择运行目标（模拟器 / 真机）",
            "Select destination (simulator / device)",
        ),
        &dest_labels,
        default_dest_idx,
    )?;
    let picked = &destinations[dest_idx];

    let rel = project
        .path
        .strip_prefix(root)
        .unwrap_or(&project.path)
        .to_string_lossy()
        .to_string();

    let defaults = load_global_file()
        .map(|f| f.defaults)
        .unwrap_or_default();

    let language = std::env::var("IOS_RUNNER_LANG").unwrap_or(defaults.language);

    Ok(RunnerConfig {
        kind: project.kind,
        path: rel,
        scheme,
        configuration: "Debug".to_string(),
        destination: picked.destination.clone(),
        derived_data: crate::global_store::global_derived_data_path(root)?
            .to_string_lossy()
            .to_string(),
        xcbeautify: defaults.xcbeautify,
        resolve_packages_before_build: defaults.resolve_packages_before_build,
        bring_simulator_to_foreground: picked.kind == DestinationKind::Simulator,
        development_team: None,
        language,
    })
}

fn print_ensure_summary(config: &RunnerConfig) {
    print_project_context(config);
    eprintln!();
    eprintln!(
        "{}",
        t(
            "配置已保存。Cmd+Shift+R 运行，Cmd+Shift+I 可重新选择设备。",
            "Configuration saved. Cmd+Shift+R to run, Cmd+Shift+I to change device.",
        )
    );
}

#[derive(Debug, Clone)]
pub struct EnsureReport {
    pub scheme: String,
    pub path: String,
    pub destination: String,
    pub wrote_config: bool,
    pub wrote_tasks: bool,
    pub has_podfile: bool,
    pub global_config: std::path::PathBuf,
}
