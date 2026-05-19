use std::process::Command;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Simulator {
    pub name: String,
    pub udid: String,
    pub runtime: String,
}

/// List available iOS simulators (iPhone / iPad), newest runtimes first.
pub fn list_simulators() -> Result<Vec<Simulator>> {
    let output = Command::new("xcrun")
        .args(["simctl", "list", "devices", "available", "-j"])
        .output()
        .context("simctl list devices")?;

    let parsed: SimctlList = serde_json::from_slice(&output.stdout).context("parse simctl JSON")?;

    let mut sims = Vec::new();
    for (runtime, devices) in parsed.devices {
        if !runtime.contains("iOS") {
            continue;
        }
        for device in devices {
            if device.is_available == Some(false) {
                continue;
            }
            if device.name.starts_with("iPhone") || device.name.starts_with("iPad") {
                sims.push(Simulator {
                    name: device.name,
                    udid: device.udid,
                    runtime: runtime.clone(),
                });
            }
        }
    }

    sims.sort_by(|a, b| b.runtime.cmp(&a.runtime).then_with(|| a.name.cmp(&b.name)));
    Ok(sims)
}

pub fn destination_for_simulator(sim: &Simulator) -> String {
    format!("platform=iOS Simulator,name={}", sim.name)
}

/// Resolve simulator UDID from an xcodebuild-style destination (`name=iPhone 17`, etc.).
pub fn udid_for_destination_name(name: &str) -> Result<String> {
    let sims = list_simulators()?;
    for sim in &sims {
        if sim.name == name {
            return Ok(sim.udid.clone());
        }
    }
    anyhow::bail!("simulator not found for name={name}");
}

#[derive(Debug, Deserialize)]
struct SimctlList {
    devices: std::collections::HashMap<String, Vec<SimDevice>>,
}

#[derive(Debug, Deserialize)]
struct SimDevice {
    name: String,
    udid: String,
    #[serde(rename = "isAvailable")]
    is_available: Option<bool>,
}
