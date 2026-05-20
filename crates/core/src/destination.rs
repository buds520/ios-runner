use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::detect::DetectedProject;
use crate::locale::t;
use crate::simulator::{destination_for_simulator, list_simulators};
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
            DestinationKind::Simulator => t("模拟器", "Simulator"),
            DestinationKind::Device => t("真机", "Device"),
        };
        format!("[{tag}] {}", self.name)
    }

    pub fn summary_line(&self) -> String {
        match self.kind {
            DestinationKind::Simulator => format!("{} · {}", t("模拟器", "Simulator"), self.name),
            DestinationKind::Device => format!("{} · {}", t("真机", "Device"), self.name),
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

    if out.is_empty() {
        out = destinations_from_simctl()?;
    }

    Ok(out)
}

/// When `xcodebuild -showdestinations` only returns placeholders (common before first Xcode open).
fn destinations_from_simctl() -> Result<Vec<RunDestination>> {
    let sims = list_simulators()?;
    Ok(sims
        .into_iter()
        .map(|sim| RunDestination {
            kind: DestinationKind::Simulator,
            name: sim.name.clone(),
            platform: "iOS Simulator".to_string(),
            destination: destination_for_simulator(&sim),
        })
        .collect())
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
        if let Some(v) = part
            .strip_prefix("platform:")
            .or_else(|| part.strip_prefix("platform="))
        {
            platform = Some(v.trim().to_string());
        } else if let Some(v) = part.strip_prefix("name:").or_else(|| part.strip_prefix("name=")) {
            name = Some(v.trim().to_string());
        } else if let Some(v) = part.strip_prefix("id:").or_else(|| part.strip_prefix("id=")) {
            id = Some(v.trim().to_string());
        } else if let Some(v) = part.strip_prefix("OS:").or_else(|| part.strip_prefix("OS=")) {
            os = Some(v.trim().to_string());
        }
    }

    let platform = platform?;
    let name = name?;

    if name.contains("placeholder") || name == "Any iOS Device" {
        return None;
    }

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

/// Human-readable device/simulator name from either `key=value` or legacy `key:value` strings.
pub fn destination_display_name(destination: &str) -> Option<String> {
    parse_destination_fields(destination).and_then(|f| f.name)
}

/// Whether this destination cannot be used for build/run (Xcode placeholders).
pub fn is_placeholder_destination(destination: &str) -> bool {
    let Some(fields) = parse_destination_fields(destination) else {
        return destination.contains("placeholder")
            || destination.contains("Any iOS Simulator Device");
    };
    fields.id.is_some_and(|id| id.contains("placeholder"))
        || fields
            .name
            .is_some_and(|n| n.contains("placeholder") || n == "Any iOS Simulator Device")
}

/// Convert stored destination to `xcodebuild -destination` form (`key=value` pairs).
pub fn normalize_xcodebuild_destination(destination: &str) -> Option<String> {
    if is_placeholder_destination(destination) {
        return None;
    }
    let fields = parse_destination_fields(destination)?;

    let platform = fields.platform?;
    let name = fields.name?;

    if platform.contains("Simulator") {
        return Some(format!("platform=iOS Simulator,name={name}"));
    }
    if platform == "iOS" {
        if let Some(id) = fields.id.filter(|i| !i.contains("placeholder")) {
            return Some(format!("platform=iOS,id={id},name={name}"));
        }
        return Some(format!("platform=iOS,name={name}"));
    }
    None
}

/// Validate before invoking xcodebuild.
pub fn validate_xcodebuild_destination(destination: &str) -> Result<()> {
    if destination.trim().is_empty() {
        bail_invalid_destination(destination, "empty")?;
    }
    if is_placeholder_destination(destination) {
        bail_invalid_destination(
            destination,
            "placeholder (not a real simulator or device)",
        )?;
    }
    if !destination.contains('=') {
        bail_invalid_destination(destination, "expected key=value pairs")?;
    }
    for part in destination.split(',') {
        let part = part.trim();
        if part.is_empty() || !part.contains('=') {
            bail_invalid_destination(destination, "malformed destination segment")?;
        }
    }
    Ok(())
}

fn bail_invalid_destination(destination: &str, reason: &str) -> Result<()> {
    anyhow::bail!(
        "{}",
        crate::locale::tf(
            || format!(
                "运行目标（destination）无效：{reason}\n  当前值: {destination}\n  请执行: ios-runner switch\n  或在 Zed 中运行「iOS-Runner: 选择 Scheme 与设备」重新选择模拟器/真机。",
            ),
            || format!(
                "Invalid run destination: {reason}\n  Current: {destination}\n  Run: ios-runner switch\n  Or use the Zed task「iOS-Runner: Select Scheme & Device」.",
            ),
        )
    )
}

struct DestinationFields {
    platform: Option<String>,
    name: Option<String>,
    id: Option<String>,
}

fn parse_destination_fields(destination: &str) -> Option<DestinationFields> {
    let mut platform = None;
    let mut name = None;
    let mut id = None;

    for part in destination.split(',') {
        let part = part.trim();
        if let Some((k, v)) = part.split_once('=') {
            apply_field(k.trim(), v.trim(), &mut platform, &mut name, &mut id);
        } else if let Some((k, v)) = part.split_once(':') {
            apply_field(k.trim(), v.trim(), &mut platform, &mut name, &mut id);
        }
    }

    if platform.is_some() || name.is_some() {
        Some(DestinationFields {
            platform,
            name,
            id,
        })
    } else {
        None
    }
}

fn apply_field(
    key: &str,
    value: &str,
    platform: &mut Option<String>,
    name: &mut Option<String>,
    id: &mut Option<String>,
) {
    match key {
        "platform" => *platform = Some(value.to_string()),
        "name" => *name = Some(value.to_string()),
        "id" => *id = Some(value.to_string()),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simulator_destination() {
        let d = parse_destination_line("{platform=iOS Simulator,name=iPhone 16,OS=18.2}").unwrap();
        assert_eq!(d.kind, DestinationKind::Simulator);
        assert_eq!(d.name, "iPhone 16");
        assert!(d.destination.contains("iPhone 16"));
    }

    #[test]
    fn parse_skips_placeholder() {
        assert!(
            parse_destination_line("{platform=iOS Simulator,name=placeholder}").is_none()
        );
    }

    #[test]
    fn validate_empty_fails() {
        assert!(validate_xcodebuild_destination("").is_err());
    }

    #[test]
    fn normalize_colon_legacy_format() {
        let fixed = normalize_xcodebuild_destination(
            "platform:iOS Simulator,name:iPhone 16",
        )
        .unwrap();
        assert_eq!(fixed, "platform=iOS Simulator,name=iPhone 16");
    }

    #[test]
    fn placeholder_rejected() {
        assert!(is_placeholder_destination(
            "platform=iOS Simulator,name=Any iOS Simulator Device"
        ));
    }
}
