use std::path::Path;

use anyhow::{Result, bail};

use crate::config::RunnerConfig;
use crate::destination::{DestinationKind, list_run_destinations};
use crate::detect::detect_project;
use crate::prompt::{confirm, pick_one};
use crate::tasks::write_zed_tasks;
use crate::xcodebuild::list_schemes;

/// When `run_after` is `Some(true)` / `Some(false)`, skip the prompt.
pub fn configure_project(root: &Path, run_after: Option<bool>) -> Result<RunnerConfig> {
    let project = detect_project(root)?;

    let schemes = list_schemes(root, &project)?;
    let scheme_labels: Vec<String> = schemes
        .iter()
        .filter(|s| !s.starts_with("Pods-"))
        .cloned()
        .collect();
    let scheme_labels = if scheme_labels.is_empty() {
        schemes
    } else {
        scheme_labels
    };

    let scheme_idx = pick_one("选择 Scheme", &scheme_labels)?;
    let scheme = scheme_labels[scheme_idx].clone();

    let destinations = list_run_destinations(root, &project, &scheme)?;
    if destinations.is_empty() {
        bail!("未找到模拟器或真机；请在 Xcode 中安装模拟器或连接设备");
    }

    let dest_labels: Vec<String> = destinations.iter().map(|d| d.menu_label()).collect();
    let dest_idx = pick_one("选择运行目标（模拟器 / 真机）", &dest_labels)?;
    let picked = &destinations[dest_idx];

    let bring_simulator = picked.kind == DestinationKind::Simulator;

    let rel = project
        .path
        .strip_prefix(root)
        .unwrap_or(&project.path)
        .to_string_lossy()
        .to_string();

    let config = RunnerConfig {
        kind: project.kind,
        path: rel,
        scheme,
        configuration: "Debug".to_string(),
        destination: picked.destination.clone(),
        derived_data: ".ios-runner/DerivedData".to_string(),
        xcbeautify: true,
        resolve_packages_before_build: true,
        bring_simulator_to_foreground: bring_simulator,
        development_team: RunnerConfig::load(root)
            .ok()
            .and_then(|c| c.development_team),
    };

    config.save(root)?;
    write_zed_tasks(root, &project)?;
    print_configure_success(&config);

    let should_run = match run_after {
        Some(v) => v,
        None => confirm("是否立即编译并运行？", true)?,
    };

    if should_run {
        eprintln!();
        eprintln!("▶ 正在启动…");
        crate::xcodebuild::run_app(root, &config)?;
        eprintln!();
        eprintln!("✓ 运行完成（Ctrl+C 可停止日志）");
    } else {
        eprintln!();
        eprintln!("已跳过运行。之后请用 Zed 任务「iOS-Runner: Run」。");
    }

    Ok(config)
}

pub fn print_configure_success(config: &RunnerConfig) {
    eprintln!();
    eprintln!("✓ 配置已保存（下次直接 Run 即可，无需重选）");
    eprintln!("  Scheme ：{}", config.scheme);
    eprintln!("  目标   ：{}", config.device_summary());
    eprintln!("  工程   ：{}", config.path);
    if config.destination.contains("Simulator") {
        eprintln!("提示：模拟器无需签名");
    } else {
        eprintln!("提示：真机需在 Xcode → Signing 中选择 Team");
    }
}
