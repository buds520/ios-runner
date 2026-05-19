use std::path::Path;

use anyhow::Result;

use crate::config::RunnerConfig;
use crate::detect::detect_project;
use crate::prompt::pick_one;
use crate::simulator::{destination_for_simulator, list_simulators};
use crate::tasks::write_zed_tasks;
use crate::xcodebuild::list_schemes;

/// Interactive scheme + simulator selection (SweetPad-style), writes config and Zed tasks.
pub fn configure_project(root: &Path) -> Result<RunnerConfig> {
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

    let scheme_idx = pick_one("Select scheme", &scheme_labels)?;
    let scheme = scheme_labels[scheme_idx].clone();

    let sims = list_simulators()?;
    let sim_labels: Vec<String> = sims
        .iter()
        .map(|s| format!("{} ({})", s.name, short_runtime(&s.runtime)))
        .collect();
    let sim_idx = pick_one("Select simulator", &sim_labels)?;
    let destination = destination_for_simulator(&sims[sim_idx]);

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
        destination,
        derived_data: ".ios-runner/DerivedData".to_string(),
        xcbeautify: true,
        resolve_packages_before_build: true,
        bring_simulator_to_foreground: true,
    };

    config.save(root)?;
    write_zed_tasks(root, &project)?;

    Ok(config)
}

fn short_runtime(runtime: &str) -> &str {
    runtime
        .strip_prefix("com.apple.CoreSimulator.SimRuntime.")
        .unwrap_or(runtime)
}
