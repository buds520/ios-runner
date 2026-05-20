use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

use crate::config::RunnerConfig;
use crate::detect::{DetectedProject, detect_project};

const CONFIG_CACHE_TTL: Duration = Duration::from_secs(5);

static CONFIG_CACHE: OnceLock<Mutex<Option<(Instant, GlobalConfigFile)>>> = OnceLock::new();

fn config_cache() -> &'static Mutex<Option<(Instant, GlobalConfigFile)>> {
    CONFIG_CACHE.get_or_init(|| Mutex::new(None))
}

fn invalidate_config_cache() {
    if let Ok(mut guard) = config_cache().lock() {
        *guard = None;
    }
}

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalConfigFile {
    #[serde(default)]
    pub defaults: GlobalDefaults,
    /// Project root (absolute path) → per-project run settings.
    #[serde(default)]
    pub projects: BTreeMap<String, RunnerConfig>,
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

/// FNV-1a 64-bit — stable across Rust versions (unlike `DefaultHasher`).
fn fnv1a64(bytes: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_OFFSET;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub fn canonical_root(root: &Path) -> PathBuf {
    root.canonicalize().unwrap_or_else(|_| root.to_path_buf())
}

/// Stable config key: canonical `.xcworkspace` / `.xcodeproj` path (shared across Open Folder parents).
pub fn project_config_key(project_path: &Path) -> String {
    project_path
        .canonicalize()
        .unwrap_or_else(|_| project_path.to_path_buf())
        .to_string_lossy()
        .to_string()
}

/// Lookup keys: project file path first, then legacy worktree root.
pub fn config_lookup_keys(root: &Path, project: &DetectedProject) -> Vec<String> {
    let mut keys = vec![project_config_key(&project.path)];
    let legacy = canonical_root(root).to_string_lossy().to_string();
    if !keys.iter().any(|k| k == &legacy) {
        keys.push(legacy);
    }
    keys
}

/// Stable cache folder name under `~/.ios-runner/DerivedData/`.
pub fn project_cache_id(root: &Path) -> String {
    let path = canonical_root(root);
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("project");
    let hash = fnv1a64(path.as_os_str().as_encoded_bytes());
    format!("{name}-{:016x}", hash)
}

pub fn global_derived_data_path(root: &Path) -> Result<PathBuf> {
    let home = dirs::home_dir().context("home directory")?;
    Ok(home
        .join(".ios-runner/DerivedData")
        .join(project_cache_id(root)))
}

pub fn load_global_file() -> Result<GlobalConfigFile> {
    if let Ok(guard) = config_cache().lock() {
        if let Some((at, file)) = guard.as_ref() {
            if at.elapsed() < CONFIG_CACHE_TTL {
                return Ok(file.clone());
            }
        }
    }

    let path = config_file_path()?;
    let file = if !path.is_file() {
        GlobalConfigFile::default()
    } else {
        let text = std::fs::read_to_string(&path).context("read global config")?;
        let mut file: GlobalConfigFile = toml::from_str(&text).context("parse global config")?;
        normalize_defaults(&mut file.defaults);
        file
    };

    if let Ok(mut guard) = config_cache().lock() {
        *guard = Some((Instant::now(), file.clone()));
    }
    Ok(file)
}

fn open_config_locked() -> Result<(PathBuf, File)> {
    let dir = config_dir()?;
    std::fs::create_dir_all(&dir).context("create ~/.config/ios-runner")?;
    let path = dir.join("config.toml");
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(&path)
        .with_context(|| format!("open {}", path.display()))?;
    file.lock_exclusive()
        .context("lock global config")?;
    Ok((path, file))
}

fn read_locked(file: &mut File) -> Result<GlobalConfigFile> {
    file.seek(std::io::SeekFrom::Start(0))
        .context("seek global config")?;
    let mut text = String::new();
    file.read_to_string(&mut text)
        .context("read global config")?;
    if text.trim().is_empty() {
        return Ok(GlobalConfigFile::default());
    }
    let mut global: GlobalConfigFile = toml::from_str(&text).context("parse global config")?;
    normalize_defaults(&mut global.defaults);
    Ok(global)
}

fn write_locked(file: &mut File, global: &GlobalConfigFile) -> Result<()> {
    let text = toml::to_string_pretty(global).context("serialize global config")?;
    file.seek(std::io::SeekFrom::Start(0))
        .context("seek global config")?;
    file.set_len(0).context("truncate global config")?;
    file.write_all(text.as_bytes())
        .context("write global config")?;
    file.sync_all().context("sync global config")?;
    Ok(())
}

/// Read-modify-write under an exclusive lock (safe for parallel Zed tasks).
pub fn update_global_file<F>(op: F) -> Result<PathBuf>
where
    F: FnOnce(&mut GlobalConfigFile) -> Result<()>,
{
    let (path, mut file) = open_config_locked()?;
    let mut global = read_locked(&mut file)?;
    op(&mut global)?;
    write_locked(&mut file, &global)?;
    file.unlock().context("unlock global config")?;
    invalidate_config_cache();
    Ok(path)
}

pub fn save_global_file(file: &GlobalConfigFile) -> Result<PathBuf> {
    let mut file = file.clone();
    normalize_defaults(&mut file.defaults);
    update_global_file(|global| {
        *global = file;
        Ok(())
    })
}

fn finish_loaded_config(mut config: RunnerConfig, root: &Path, defaults: &GlobalDefaults) -> Result<RunnerConfig> {
    config.derived_data = global_derived_data_path(root)?
        .to_string_lossy()
        .to_string();
    config.merge_defaults(defaults);
    config.normalize();
    Ok(config)
}

/// Load config: global store first, then legacy project `.ios-runner.toml`.
pub fn load_config(root: &Path) -> Result<RunnerConfig> {
    let project = detect_project(root)?;
    load_config_for_project(root, &project)
}

pub fn load_config_for_project(root: &Path, project: &DetectedProject) -> Result<RunnerConfig> {
    let file = load_global_file()?;
    for key in config_lookup_keys(root, project) {
        if let Some(config) = file.projects.get(&key).cloned() {
            return finish_loaded_config(config, root, &file.defaults);
        }
    }

    if let Ok(local) = RunnerConfig::load_local(root) {
        return finish_loaded_config(local, root, &file.defaults);
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
    let project = detect_project(root)?;
    save_config_for_project(root, &project, config)
}

pub fn save_config_for_project(
    root: &Path,
    project: &DetectedProject,
    config: &RunnerConfig,
) -> Result<PathBuf> {
    let key = project_config_key(&project.path);
    let mut stored = config.clone();
    stored.normalize();
    stored.derived_data = global_derived_data_path(root)?
        .to_string_lossy()
        .to_string();

    update_global_file(|global| {
        global.projects.insert(key, stored);
        Ok(())
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_cache_id_stable() {
        let a = project_cache_id(Path::new("/tmp/foo"));
        let b = project_cache_id(Path::new("/tmp/foo"));
        assert_eq!(a, b);
        assert!(a.starts_with("foo-"));
    }
}
