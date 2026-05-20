use std::path::Path;

use anyhow::{Result, bail};

use crate::config::RunnerConfig;
use crate::destination::{DestinationKind, list_run_destinations};
use crate::detect::{detect_project, filter_schemes_for_project, pick_default_scheme, assert_ios_project};
use crate::locale::t;
use crate::prompt::{confirm, is_interactive_tty, pick_one_with_default};
use crate::global_store::{load_global_file};
use crate::tasks::write_zed_tasks;
use crate::xcodebuild::list_schemes;

/// When `run_after` is `Some(true)` / `Some(false)`, skip the prompt.
pub fn configure_project(root: &Path, run_after: Option<bool>) -> Result<RunnerConfig> {
    let project = detect_project(root)?;
    assert_ios_project(root, &project)?;

    let schemes = list_schemes(root, &project)?;
    let scheme_labels = filter_schemes_for_project(&schemes, &project);
    let default_scheme = pick_default_scheme(&schemes, &project)
        .unwrap_or_else(|| scheme_labels.first().cloned().unwrap_or_default());
    let default_scheme_idx = scheme_labels
        .iter()
        .position(|s| s == &default_scheme)
        .unwrap_or(0);

    let scheme_idx = if is_interactive_tty() {
        pick_one_with_default(
            t("选择 Scheme", "Select scheme"),
            &scheme_labels,
            default_scheme_idx,
        )?
    } else {
        default_scheme_idx
    };
    let scheme = scheme_labels[scheme_idx].clone();

    let destinations = list_run_destinations(root, &project, &scheme)?;
    if destinations.is_empty() {
        bail!(
            "{}",
            t(
                "未找到模拟器或真机；请在 Xcode 中安装模拟器或连接设备",
                "No simulator or device found. Install a simulator in Xcode or connect a device.",
            )
        );
    }

    let dest_labels: Vec<String> = destinations.iter().map(|d| d.menu_label()).collect();
    let default_dest_idx = destinations
        .iter()
        .position(|d| d.kind == DestinationKind::Device)
        .unwrap_or(0);
    let dest_idx = if is_interactive_tty() {
        pick_one_with_default(
            t(
                "选择运行目标（模拟器 / 真机）",
                "Select destination (simulator / device)",
            ),
            &dest_labels,
            default_dest_idx,
        )?
    } else {
        default_dest_idx
    };
    let picked = &destinations[dest_idx];

    let bring_simulator = picked.kind == DestinationKind::Simulator;

    let rel = project
        .path
        .strip_prefix(root)
        .unwrap_or(&project.path)
        .to_string_lossy()
        .to_string();

    let language = std::env::var("IOS_RUNNER_LANG").unwrap_or_else(|_| {
        load_global_file()
            .map(|f| f.defaults.language)
            .unwrap_or_else(|_| "zh-CN".to_string())
    });

    let config = RunnerConfig {
        kind: project.kind,
        path: rel,
        scheme,
        configuration: "Debug".to_string(),
        destination: picked.destination.clone(),
        derived_data: crate::global_store::global_derived_data_path(root)?
            .to_string_lossy()
            .to_string(),
        xcbeautify: true,
        resolve_packages_before_build: true,
        bring_simulator_to_foreground: bring_simulator,
        development_team: RunnerConfig::load(root)
            .ok()
            .and_then(|c| c.development_team),
        language,
    };

    let global_path = config.save(root)?;
    config.apply_locale();
    let tasks_root = project
        .path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| root.to_path_buf());
    if crate::tasks::should_refresh_project_tasks(&tasks_root.join(".zed/tasks.json")) {
        write_zed_tasks(&tasks_root, &project)?;
    }
    print_configure_success(&config, &global_path);

    let should_run = match run_after {
        Some(v) => v,
        None => confirm(
            t("是否立即编译并运行？", "Build and run now?"),
            true,
        )?,
    };

    if should_run {
        eprintln!();
        eprintln!("{}", t("▶ 正在启动…", "▶ Starting…"));
        crate::xcodebuild::run_app(root, &config)?;
        eprintln!();
        eprintln!(
            "{}",
            t(
                "✓ 运行完成（Ctrl+C 可停止日志）",
                "✓ Run finished (Ctrl+C to stop log stream)",
            )
        );
    } else {
        eprintln!();
        eprintln!(
            "{}",
            t(
                "已跳过运行。之后请用 Zed 任务「iOS-Runner: Run」。",
                "Skipped run. Use Zed task「iOS-Runner: Run」later.",
            )
        );
    }

    Ok(config)
}

pub fn print_configure_success(config: &RunnerConfig, global_path: &std::path::Path) {
    eprintln!();
    eprintln!(
        "{}",
        t(
            "✓ 配置已保存到全局（不修改工程目录）",
            "✓ Saved to global config (project tree unchanged)",
        )
    );
    eprintln!("  {} : {}", t("配置", "Config"), global_path.display());
    eprintln!("  Scheme : {}", config.scheme);
    eprintln!(
        "  {} : {}",
        t("目标", "Destination"),
        config.device_summary()
    );
    eprintln!("  {} : {}", t("工程", "Project"), config.path);
    if config.destination.contains("Simulator") {
        eprintln!(
            "{}",
            t("提示：模拟器无需签名", "Tip: no signing required for simulator")
        );
    } else {
        eprintln!(
            "{}",
            t(
                "提示：真机需在 Xcode → Signing 中选择 Team",
                "Tip: select a Team under Xcode → Signing for device builds",
            )
        );
    }
}
