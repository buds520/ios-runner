mod bootstrap;
mod util;
mod device;
mod locale;
mod global_store;
mod global_tasks;
mod zed_keymap;
mod build_settings;
mod build_diagnostics;
mod config;
mod configure;
mod destination;
mod detect;
mod ensure;
mod platform;
mod prompt;
mod simulator;
mod switch;
mod tasks;
mod terminal_ui;
mod uninstall;
mod xcodebuild;

pub use config::{ProjectKind, RunnerConfig};
pub use bootstrap::INSTALL_DIR;
pub use global_store::{config_file_path, load_global_file, save_global_file};
pub use global_tasks::{
    global_tasks_json_pretty, global_zed_tasks_contain_legacy_scripts, global_zed_tasks_ready,
    install_global_zed_tasks, uninstall_global_zed_tasks, zed_config_dir,
};
pub use uninstall::{UninstallOptions, UninstallReport, uninstall_ios_runner};
pub use zed_keymap::{
    embedded_keymap_entry, install_global_zed_keymap, uninstall_global_zed_keymap,
};
pub use configure::{configure_project, print_configure_success};
pub use destination::{
    DestinationKind, RunDestination, is_macos_destination, is_placeholder_destination,
    is_simulator_destination, list_run_destinations, validate_xcodebuild_destination,
};
pub use detect::{
    DetectedProject, create_config, detect_project, filter_schemes_for_project, pick_default_scheme,
};
pub use platform::{platforms_macos_only, scheme_is_macos_only, supported_platforms_for_scheme};
pub use ensure::{EnsureReport, ensure_project};
pub use switch::switch_destination;
pub use locale::{Lang, init_locale, lang, set_lang, t, tf};
pub use util::has_xcbeautify;
pub use simulator::{Simulator, list_simulators};
pub use tasks::write_zed_tasks;
pub use xcodebuild::{
    build_project, detect_incremental_fresh, list_schemes, resolve_packages, run_app,
    run_on_device, run_on_mac, run_on_simulator,
};
