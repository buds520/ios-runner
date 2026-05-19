use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectKind {
    Workspace,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerConfig {
    pub kind: ProjectKind,
    pub path: String,
    pub scheme: String,
    #[serde(default = "default_configuration")]
    pub configuration: String,
    pub destination: String,
    #[serde(default = "default_derived_data")]
    pub derived_data: String,
    #[serde(default)]
    pub xcbeautify: bool,
    #[serde(default = "default_resolve_packages")]
    pub resolve_packages_before_build: bool,
    #[serde(default = "default_true")]
    pub bring_simulator_to_foreground: bool,
    /// Apple Developer Team ID (required for physical device builds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub development_team: Option<String>,
}

fn default_resolve_packages() -> bool {
    true
}

fn default_true() -> bool {
    true
}

fn default_configuration() -> String {
    "Debug".to_string()
}

fn default_derived_data() -> String {
    ".ios-runner/DerivedData".to_string()
}

impl RunnerConfig {
    pub const FILE_NAME: &'static str = ".ios-runner.toml";
    const LEGACY_FILE_NAME: &'static str = ".xcode-pilot.toml";

    pub fn load(root: &Path) -> Result<Self> {
        let path = config_path(root)?;
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("missing config; run `ios-runner init` ({})", path.display()))?;
        let mut config: RunnerConfig = toml::from_str(&text).context("parse ios-runner config")?;
        config.normalize();
        Ok(config)
    }

    /// Fix legacy paths / destinations written by older ios-runner versions.
    pub fn normalize(&mut self) {
        if self.path.ends_with(".xcodeproj/project.xcworkspace") {
            self.path = self
                .path
                .trim_end_matches("/project.xcworkspace")
                .to_string();
            self.kind = ProjectKind::Project;
        }

        if self.destination.contains("Simulator") && self.destination.contains("id=") {
            if let Some(name) = destination_name(&self.destination) {
                self.destination = format!("platform=iOS Simulator,name={name}");
            }
        }
    }

    pub fn save(&self, root: &Path) -> Result<()> {
        let mut config = self.clone();
        config.normalize();
        let path = root.join(Self::FILE_NAME);
        let text = toml::to_string_pretty(&config).context("serialize config")?;
        std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))
    }

    pub fn device_summary(&self) -> String {
        let name = destination_name(&self.destination).unwrap_or_else(|| "?".into());
        if self.destination.contains("Simulator") {
            format!("模拟器 {name}")
        } else {
            format!("真机 {name}")
        }
    }

    pub fn project_path(&self, root: &Path) -> PathBuf {
        root.join(&self.path)
    }

    pub fn derived_data_path(&self, root: &Path) -> PathBuf {
        root.join(&self.derived_data)
    }

    pub fn validate(&self, root: &Path) -> Result<()> {
        let project = self.project_path(root);
        if !project.exists() {
            bail!("project path does not exist: {}", project.display());
        }
        Ok(())
    }
}

fn destination_name(destination: &str) -> Option<String> {
    destination.split(',').find_map(|part| {
        let part = part.trim();
        part.strip_prefix("name=")
            .map(|s| s.trim().to_string())
    })
}

fn config_path(root: &Path) -> Result<PathBuf> {
    let primary = root.join(RunnerConfig::FILE_NAME);
    if primary.is_file() {
        return Ok(primary);
    }
    let legacy = root.join(RunnerConfig::LEGACY_FILE_NAME);
    if legacy.is_file() {
        return Ok(legacy);
    }
    Ok(primary)
}
