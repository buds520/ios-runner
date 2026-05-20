use std::path::Path;

use anyhow::{Result, bail};

use crate::destination::list_run_destinations;
use crate::detect::detect_project;
use crate::global_store::{load_config_for_project, save_config_for_project};
use crate::locale::t;
use crate::prompt::{is_interactive_tty, pick_one_with_default};

/// List available run destinations for the current project (JSON to stdout when `list_only`).
pub fn switch_destination(root: &Path, list_only: bool) -> Result<()> {
    let project = detect_project(root)?;
    let config = load_config_for_project(root, &project)?;

    let destinations = list_run_destinations(root, &project, &config.scheme)?;
    if destinations.is_empty() {
        bail!(
            "{}",
            t(
                "未找到运行目标",
                "No run destination found",
            )
        );
    }

    if list_only {
        println!("{}", serde_json::to_string_pretty(&destinations)?);
        return Ok(());
    }

    if !is_interactive_tty() {
        bail!(
            "{}",
            t(
                "非交互环境请使用 ios-runner switch --list 查看设备",
                "Use ios-runner switch --list in non-interactive mode",
            )
        );
    }

    let labels: Vec<String> = destinations.iter().map(|d| d.menu_label()).collect();
    let current_idx = destinations
        .iter()
        .position(|d| d.destination == config.destination)
        .unwrap_or(0);

    let idx = pick_one_with_default(
        t("切换运行目标", "Switch destination"),
        &labels,
        current_idx,
    )?;
    let picked = &destinations[idx];

    let mut updated = config;
    updated.destination = picked.destination.clone();
    updated.bring_simulator_to_foreground =
        picked.kind == crate::destination::DestinationKind::Simulator;
    let path = save_config_for_project(root, &project, &updated)?;
    updated.apply_locale();

    eprintln!();
    eprintln!(
        "{}",
        t("✓ 运行目标已更新", "✓ Destination updated"),
    );
    eprintln!("  {} : {}", t("配置", "Config"), path.display());
    eprintln!("  {} : {}", t("目标", "Destination"), updated.device_summary());
    Ok(())
}
