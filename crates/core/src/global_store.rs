use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::config::RunnerConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalDefaults {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_true")]
    pub xcbeautify: bool,
    #[serde(default = "default_resolve_packages")]
    pub resolve_packages_before_build: bool,
    #[serde(default = "default_true")]
    pub bring_simulator_to_foreground: bool,
}

fn default_language() -> String {
    "zh-CN".to_string()
}

fn default_true() -> bool {
    true
}

fn default_resolve_packages() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfigFile {
    #[serde(default)]
    pub defaults: GlobalDefaults,
    /// Project root (absolute path) → per-project run settings.
    #[serde(default)]
    pub projects: BTreeMap<String, RunnerConfig>,
}

impl Default for GlobalConfigFile {
    fn default() -> Self {
        Self {
            defaults: GlobalDefaults::default(),
            projects: BTreeMap::new(),
        }
    }
}

pub fn config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("home directory")?;
    Ok(home.join(".config/ios-runner"))
}

pub fn config_file_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

fn normalize_defaults(defaults: &mut GlobalDefaults) {
    if defaults.language.trim().is_empty() {
        defaults.language = default_language();
    }
}

pub fn load_global_file() -> Result<GlobalConfigFile> {
    let path = config_file_path()?;
    if !path.is_file() {
        return Ok(GlobalConfigFile::default());
    }
    let text = std::fs::read_to_string(&path).context("read global config")?;
    let mut file: GlobalConfigFile = toml::from_str(&text).context("parse global config")?;
    normalize_defaults(&mut file.defaults);
    Ok(file)
}

pub fn save_global_file(file: &GlobalConfigFile) -> Result<PathBuf> {
    let mut file = file.clone();
    normalize_defaults(&mut file.defaults);
    let dir = config_dir()?;
    std::fs::create_dir_all(&dir).context("create ~/.config/ios-runner")?;
    let path = dir.join("config.toml");
    let text = toml::to_string_pretty(&file).context("serialize global config")?;
    std::fs::write(&path, &text).context("write global config")?;
    Ok(path)
}

pub fn canonical_root(root: &Path) -> PathBuf {
    root.canonicalize().unwrap_or_else(|_| root.to_path_buf())
}

/// Stable cache folder name under `~/.ios-runner/DerivedData/`.
pub fn project_cache_id(root: &Path) -> String {
    let path = canonical_root(root);
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project");
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut hasher);
    format!("{name}-{:016x}", hasher.finish())
}

pub fn global_derived_data_path(root: &Path) -> Result<PathBuf> {
    let home = dirs::home_dir().context("home directory")?;
    Ok(home
        .join(".ios-runner/DerivedData")
        .join(project_cache_id(root)))
}

/// Load config: global store first, then legacy project `.ios-runner.toml`.
pub fn load_config(root: &Path) -> Result<RunnerConfig> {
    let key = canonical_root(root).to_string_lossy().to_string();
    let file = load_global_file()?;

    if let Some(mut config) = file.projects.get(&key).cloned() {
        config.derived_data = global_derived_data_path(root)?
            .to_string_lossy()
            .to_string();
        config.normalize();
        return Ok(config);
    }

    if let Ok(mut local) = RunnerConfig::load_local(root) {
        local.derived_data = global_derived_data_path(root)?
            .to_string_lossy()
            .to_string();
        return Ok(local);
    }

    bail!(
        "{}",
        crate::locale::t(
            "未找到此工程的 iOS-Runner 配置。请运行: ios-runner ensure 或 ios-runner configure",
            "No iOS-Runner config for this project. Run: ios-runner ensure or ios-runner configure",
        )
    )
}

/// Save to `~/.config/ios-runner/config.toml` only (does not touch the project tree).
pub fn save_config(root: &Path, config: &RunnerConfig) -> Result<PathBuf> {
    let key = canonical_root(root).to_string_lossy().to_string();
    let mut file = load_global_file()?;
    let mut stored = config.clone();
    stored.normalize();
    stored.derived_data = global_derived_data_path(root)?
        .to_string_lossy()
        .to_string();
    file.projects.insert(key, stored);
    save_global_file(&file)
}

pub fn should_write_project_tasks() -> bool {
    std::env::var("IOS_RUNNER_WRITE_PROJECT_TASKS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn should_write_local_config() -> bool {
    std::env::var("IOS_RUNNER_LOCAL_CONFIG")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}
