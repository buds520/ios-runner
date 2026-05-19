mod bootstrap;
mod global_tasks;
mod build_settings;
mod config;
mod configure;
mod destination;
mod detect;
mod ensure;
mod prompt;
mod simulator;
mod tasks;
mod terminal_ui;
mod xcodebuild;

pub use config::{ProjectKind, RunnerConfig};
pub use bootstrap::INSTALL_DIR;
pub use global_tasks::install_global_zed_tasks;
pub use configure::{configure_project, print_configure_success};
pub use destination::{DestinationKind, RunDestination, list_run_destinations};
pub use detect::{DetectedProject, create_config, detect_project};
pub use ensure::{EnsureReport, ensure_project};
pub use simulator::{Simulator, list_simulators};
pub use tasks::write_zed_tasks;
pub use xcodebuild::{
    build_project, list_schemes, resolve_packages, run_app, run_on_device, run_on_simulator,
};
