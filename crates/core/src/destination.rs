use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::detect::DetectedProject;
use crate::xcodebuild::add_project_args;

/// A build/run target from `xcodebuild -showdestinations`.
#[derive(Debug, Clone, Serialize)]
pub struct RunDestination {
    pub kind: DestinationKind,
    pub name: String,
    pub platform: String,
    /// Xcode destination string for `-destination`.
    pub destination: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DestinationKind {
    Simulator,
    Device,
}

impl RunDestination {
    pub fn menu_label(&self) -> String {
        let tag = match self.kind {
            DestinationKind::Simulator => "模拟器",
            DestinationKind::Device => "真机",
        };
        format!("[{tag}] {}", self.name)
    }

    pub fn summary_line(&self) -> String {
        match self.kind {
            DestinationKind::Simulator => format!("模拟器 · {}", self.name),
            DestinationKind::Device => format!("真机 · {}", self.name),
        }
    }
}

/// List simulators and connected physical iOS devices for a scheme.
pub fn list_run_destinations(
    root: &Path,
    project: &DetectedProject,
    scheme: &str,
) -> Result<Vec<RunDestination>> {
    let mut cmd = std::process::Command::new("xcodebuild");
    add_project_args(&mut cmd, project);
    cmd.args(["-scheme", scheme, "-showdestinations"]);

    let output = cmd
        .current_dir(root)
        .output()
        .context("xcodebuild -showdestinations")?;

    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('{') || !trimmed.contains('}') {
            continue;
        }
        if let Some(dest) = parse_destination_line(trimmed) {
            if !dest.name.contains("placeholder") && dest.name != "Any iOS Device" {
                out.push(dest);
            }
        }
    }

    // Prefer simulators first, then devices; stable name order within kind.
    out.sort_by(|a, b| {
        a.kind
            .cmp(&b.kind)
            .then_with(|| a.name.cmp(&b.name))
    });

    Ok(out)
}

fn parse_destination_line(line: &str) -> Option<RunDestination> {
    let start = line.find('{')? + 1;
    let end = line.rfind('}')?;
    let inner = line[start..end].trim();
    if inner.is_empty() {
        return None;
    }

    let mut platform = None;
    let mut name = None;
    let mut id = None;
    let mut os = None;

    for part in inner.split(',') {
        let part = part.trim();
        if let Some(v) = part.strip_prefix("platform:") {
            platform = Some(v.trim().to_string());
        } else if let Some(v) = part.strip_prefix("name:") {
            name = Some(v.trim().to_string());
        } else if let Some(v) = part.strip_prefix("id:") {
            id = Some(v.trim().to_string());
        } else if let Some(v) = part.strip_prefix("OS:") {
            os = Some(v.trim().to_string());
        }
    }

    let platform = platform?;
    let name = name?;

    if platform == "macOS" {
        return None;
    }

    let kind = if platform.contains("Simulator") {
        DestinationKind::Simulator
    } else if platform == "iOS" {
        DestinationKind::Device
    } else {
        return None;
    };

    // Simulators: name-only destination is most reliable with xcodebuild.
    // Devices: include id for install/launch.
    let destination = if platform.contains("Simulator") {
        let _ = (id, os);
        format!("platform=iOS Simulator,name={name}")
    } else if let Some(id) = id.filter(|i| !i.contains("placeholder")) {
        format!("platform=iOS,id={id},name={name}")
    } else {
        format!("platform=iOS,name={name}")
    };

    Some(RunDestination {
        kind,
        name,
        platform,
        destination,
    })
}

pub fn device_udid_from_destination(destination: &str) -> Result<String> {
    for part in destination.split(',') {
        let part = part.trim();
        if let Some(id) = part.strip_prefix("id=") {
            let id = id.trim();
            if !id.is_empty() && !id.contains("placeholder") {
                return Ok(id.to_string());
            }
        }
    }
    anyhow::bail!("no device id in destination: {destination}")
}

pub fn is_simulator_destination(destination: &str) -> bool {
    destination.contains("Simulator")
}
