mod build_settings;
mod config;
mod configure;
mod detect;
mod ensure;
mod prompt;
mod simulator;
mod tasks;
mod xcodebuild;

pub use config::{ProjectKind, RunnerConfig};
pub use configure::configure_project;
pub use detect::{DetectedProject, create_config, detect_project};
pub use ensure::{EnsureReport, ensure_project};
pub use simulator::{Simulator, list_simulators};
pub use tasks::write_zed_tasks;
pub use xcodebuild::{build_project, list_schemes, resolve_packages, run_on_simulator};
