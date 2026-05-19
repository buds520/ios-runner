mod build_settings;
mod config;
mod detect;
mod ensure;
mod tasks;
mod xcodebuild;

pub use config::RunnerConfig;
pub use detect::{DetectedProject, create_config, detect_project};
pub use ensure::{EnsureReport, ensure_project};
pub use tasks::write_zed_tasks;
pub use xcodebuild::{build_project, list_schemes, resolve_packages, run_on_simulator};
