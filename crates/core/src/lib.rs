mod bootstrap;
mod build_diagnostics;
mod build_settings;
mod config;
mod configure;
mod destination;
mod detect;
mod device;
mod ensure;
mod global_store;
mod global_tasks;
mod locale;
mod platform;
mod prompt;
mod simulator;
mod switch;
mod tasks;
mod terminal_ui;
mod uninstall;
mod util;
mod xcodebuild;
mod zed_keymap;

pub use bootstrap::INSTALL_DIR;
pub use config::{ProjectKind, RunnerConfig};
pub use configure::{configure_project, print_configure_success};
pub use destination::{
    is_macos_destination, is_placeholder_destination, is_simulator_destination,
    list_run_destinations, validate_xcodebuild_destination, DestinationKind, RunDestination,
};
pub use detect::{
    create_config, detect_project, filter_schemes_for_project, pick_default_scheme, DetectedProject,
};
pub use ensure::{ensure_project, repair_saved_destination, EnsureReport};
pub use global_store::{config_file_path, load_global_file, save_global_file};
pub use global_tasks::{
    global_tasks_json_pretty, global_zed_tasks_contain_legacy_scripts, global_zed_tasks_ready,
    install_global_zed_tasks, uninstall_global_zed_tasks, zed_config_dir,
};
pub use locale::{init_locale, lang, set_lang, t, tf, Lang};
pub use platform::{
    platforms_macos_only, platforms_support_ios, scheme_is_macos_only,
    supported_platforms_for_scheme,
};
pub use simulator::{list_simulators, Simulator};
pub use switch::switch_destination;
pub use tasks::write_zed_tasks;
pub use uninstall::{uninstall_ios_runner, UninstallOptions, UninstallReport};
pub use util::has_xcbeautify;
pub use xcodebuild::{
    build_project, detect_incremental_fresh, list_schemes, resolve_packages, run_app,
    run_on_device, run_on_mac, run_on_simulator,
};
pub use zed_keymap::{
    embedded_keymap_entry, install_global_zed_keymap, uninstall_global_zed_keymap,
};
