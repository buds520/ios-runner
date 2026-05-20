use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
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
    #[serde(default = "default_true")]
    pub xcbeautify: bool,
    #[serde(default = "default_resolve_packages")]
    pub resolve_packages_before_build: bool,
    #[serde(default = "default_true")]
    pub bring_simulator_to_foreground: bool,
    /// Apple Developer Team ID (required for physical device builds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub development_team: Option<String>,
    /// Terminal messages: `zh-CN` (default) or `en`. Override with env `IOS_RUNNER_LANG`.
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "zh-CN".to_string()
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

    /// Prefer global `~/.config/ios-runner/config.toml`; see `global_store::load_config`.
    pub fn load(root: &Path) -> Result<Self> {
        crate::global_store::load_config(root)
    }

    /// Read legacy project-local config only.
    pub fn load_local(root: &Path) -> Result<Self> {
        let path = local_config_path(root)?;
        let text =
            std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
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

        if let Some(fixed) = crate::destination::normalize_xcodebuild_destination(&self.destination)
        {
            self.destination = fixed;
        }
    }

    /// Save to global config; optionally mirror to `.ios-runner.toml` if `IOS_RUNNER_LOCAL_CONFIG=1`.
    pub fn save(&self, root: &Path) -> Result<PathBuf> {
        let path = crate::global_store::save_config(root, self)?;
        if crate::global_store::should_write_local_config() {
            self.save_local(root)?;
        }
        Ok(path)
    }

    pub fn save_local(&self, root: &Path) -> Result<()> {
        let mut config = self.clone();
        config.normalize();
        let path = root.join(Self::FILE_NAME);
        let text = toml::to_string_pretty(&config).context("serialize config")?;
        std::fs::write(&path, text).with_context(|| format!("write {}", path.display()))?;
        Ok(())
    }

    pub fn device_summary(&self) -> String {
        use crate::locale::t;
        let name = crate::destination::destination_display_name(&self.destination)
            .unwrap_or_else(|| "?".into());
        if self.destination.contains("macOS") {
            format!("{} · {name}", t("Mac", "Mac"))
        } else if self.destination.contains("Simulator") || self.destination.contains("Simulator:")
        {
            format!("{} · {name}", t("模拟器", "Simulator"),)
        } else {
            format!("{} · {name}", t("真机", "Device"))
        }
    }

    pub fn apply_locale(&self) {
        if std::env::var("IOS_RUNNER_LANG").is_err() {
            crate::locale::set_lang(crate::locale::Lang::parse(&self.language));
        }
    }

    /// Fill unset fields from global defaults (after loading a project entry).
    pub fn merge_defaults(&mut self, defaults: &crate::global_store::GlobalDefaults) {
        if self.language.trim().is_empty() {
            self.language = defaults.language.clone();
        }
        // Booleans in stored config are always explicit; defaults apply at create time.
        let _ = defaults;
    }

    pub fn project_path(&self, root: &Path) -> PathBuf {
        root.join(&self.path)
    }

    pub fn derived_data_path(&self, _root: &Path) -> PathBuf {
        PathBuf::from(&self.derived_data)
    }

    pub fn validate(&self, root: &Path) -> Result<()> {
        let project = self.project_path(root);
        if !project.exists() {
            bail!("project path does not exist: {}", project.display());
        }
        crate::destination::validate_xcodebuild_destination(&self.destination)?;
        Ok(())
    }
}

fn local_config_path(root: &Path) -> Result<PathBuf> {
    let primary = root.join(RunnerConfig::FILE_NAME);
    if primary.is_file() {
        return Ok(primary);
    }
    let legacy = root.join(RunnerConfig::LEGACY_FILE_NAME);
    if legacy.is_file() {
        return Ok(legacy);
    }
    bail!(
        "{}",
        crate::locale::t("缺少 .ios-runner.toml", "missing .ios-runner.toml",)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_project_xcworkspace() {
        let mut c = RunnerConfig {
            kind: ProjectKind::Workspace,
            path: "Demo.xcodeproj/project.xcworkspace".into(),
            scheme: "App".into(),
            configuration: "Debug".into(),
            destination: "platform=iOS Simulator,name=iPhone 16".into(),
            derived_data: ".ios-runner/DerivedData".into(),
            xcbeautify: true,
            resolve_packages_before_build: true,
            bring_simulator_to_foreground: true,
            development_team: None,
            language: "zh-CN".into(),
        };
        c.normalize();
        assert_eq!(c.path, "Demo.xcodeproj");
        assert_eq!(c.kind, ProjectKind::Project);
    }
}
