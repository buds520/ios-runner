use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use ios_runner_core::{
    RunnerConfig, build_project, configure_project, detect_project, embedded_keymap_entry,
    ensure_project, global_tasks_json_pretty, global_zed_tasks_contain_legacy_scripts,
    init_locale, install_global_zed_keymap, install_global_zed_tasks, is_placeholder_destination,
    list_run_destinations, list_schemes, list_simulators, resolve_packages, run_app, t, tf,
    switch_destination, uninstall_ios_runner, zed_config_dir, UninstallOptions,
};

mod mcp;

#[derive(Parser)]
#[command(name = "ios-runner", about = "Build and run iOS Xcode projects in Zed (iOS-Runner)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Doctor,
    /// Write config + Zed tasks (auto-detect scheme/simulator)
    Init {
        /// Interactive scheme and simulator picker
        #[arg(long, short)]
        pick: bool,
    },
    Ensure {
        /// No output when project is already configured
        #[arg(short, long)]
        quiet: bool,
    },
    /// Interactive scheme + device; saves config (prompts to run unless flags set)
    Configure {
        /// After saving, build and launch immediately
        #[arg(long, short)]
        run: bool,
        /// Only save config, do not run
        #[arg(long)]
        no_run: bool,
    },
    Mcp,
    Build {
        /// Full xcodebuild output (no xcbeautify)
        #[arg(long, short)]
        verbose: bool,
    },
    Run {
        #[arg(long, short)]
        verbose: bool,
    },
    ResolvePackages,
    /// Copy this executable to ~/.ios-runner/bin/ios-runner
    InstallSelf,
    /// Add iOS-Runner tasks to ~/.config/zed/tasks.json (all projects)
    InstallZedTasks {
        #[arg(short, long)]
        quiet: bool,
    },
    /// Print global Zed tasks JSON (for extension embed; stdout only)
    EmitGlobalTasksJson,
    /// Print global keymap entry JSON (for extension embed; stdout only)
    EmitEmbeddedKeymapJson,
    /// Remove CLI, Zed tasks/keymap, and global config from this machine
    Uninstall {
        /// Keep ~/.config/ios-runner/ (scheme/device settings)
        #[arg(long)]
        keep_config: bool,
        /// Also delete ~/.ios-runner/DerivedData/ (build cache)
        #[arg(long)]
        purge_derived_data: bool,
    },
    List {
        #[arg(long, default_value = "schemes")]
        what: String,
    },
    /// Switch simulator/device (interactive) or list destinations
    Switch {
        #[arg(long)]
        list: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = workspace_root()?;
    init_locale(Some(&root));

    match cli.command {
        Commands::Doctor => cmd_doctor(&root),
        Commands::Init { pick } => cmd_init(&root, pick),
        Commands::Ensure { quiet } => cmd_init_ensure(&root, quiet),
        Commands::Configure { run, no_run } => cmd_configure(&root, run, no_run),
        Commands::Mcp => mcp::run_mcp(),
        Commands::InstallSelf => cmd_install_self(),
        Commands::InstallZedTasks { quiet } => cmd_install_zed_tasks(quiet),
        Commands::EmitGlobalTasksJson => {
            print!("{}", global_tasks_json_pretty()?);
            Ok(())
        }
        Commands::EmitEmbeddedKeymapJson => {
            print!(
                "{}",
                serde_json::to_string_pretty(&embedded_keymap_entry()).context("serialize keymap")?
            );
            Ok(())
        }
        Commands::Uninstall {
            keep_config,
            purge_derived_data,
        } => cmd_uninstall(keep_config, purge_derived_data),
        Commands::Build { verbose } => {
            set_verbose_logs(verbose);
            let config = load_config(&root)?;
            build_project(&root, &config)
        }
        Commands::Run { verbose } => {
            set_verbose_logs(verbose);
            let config = load_config(&root)?;
            run_app(&root, &config)
        }
        Commands::ResolvePackages => {
            let config = load_config(&root)?;
            resolve_packages(&root, &config)
        }
        Commands::List { what } => cmd_list(&root, &what),
        Commands::Switch { list } => switch_destination(&root, list),
    }
}

fn workspace_root() -> Result<PathBuf> {
    env::current_dir().context("current directory")
}

fn load_config(root: &Path) -> Result<RunnerConfig> {
    let config = RunnerConfig::load(root)?;
    config.apply_locale();
    config.validate(root)?;
    Ok(config)
}

fn set_verbose_logs(verbose: bool) {
    if verbose {
        std::env::set_var("IOS_RUNNER_RAW_LOG", "1");
    }
}

fn cmd_install_zed_tasks(quiet: bool) -> Result<()> {
    let tasks = install_global_zed_tasks()?;
    warn_legacy_project_tasks()?;
    let _keymap = install_global_zed_keymap()?;
    if quiet {
        return Ok(());
    }
    eprintln!(
        "{} {}",
        t("✓ iOS-Runner 任务已写入", "✓ Wrote iOS-Runner tasks to"),
        tasks.display()
    );
    eprintln!(
        "{}",
        t(
            "  快捷键: Cmd+Shift+R 运行  B 编译  I 选设备  U 初始化",
            "  Keys: Cmd+Shift+R run  B build  I device  U setup",
        )
    );
    Ok(())
}

fn warn_legacy_project_tasks() -> Result<()> {
    let cwd = env::current_dir().context("current directory")?;
    let path = cwd.join(".zed/tasks.json");
    if !path.is_file() {
        return Ok(());
    }
    let text = std::fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    if text.contains("正在下载命令行工具")
        || text.contains("curl -fsSL")
        || text.contains("$ir_bin")
        || text.contains("$IOS_RUNNER")
    {
        eprintln!(
            "{}",
            tf(
                || format!(
                    "⚠ 工程内 {} 仍是旧版下载脚本，会覆盖全局任务体验。可删除该文件或设置 IOS_RUNNER_WRITE_PROJECT_TASKS=1 后执行 ios-runner ensure 重写。",
                    path.display()
                ),
                || format!(
                    "⚠ Project {} still uses legacy download scripts. Delete it or run `ios-runner ensure` with IOS_RUNNER_WRITE_PROJECT_TASKS=1.",
                    path.display()
                ),
            )
        );
    }
    Ok(())
}

fn cmd_uninstall(keep_config: bool, purge_derived_data: bool) -> Result<()> {
    let report = uninstall_ios_runner(&UninstallOptions {
        keep_config,
        purge_derived_data,
    })?;

    eprintln!("{}", t("已卸载 iOS-Runner：", "Uninstalled iOS-Runner:"));
    if report.removed.is_empty() {
        eprintln!(
            "{}",
            t("  （未发现已安装的文件）", "  (nothing to remove)")
        );
    } else {
        for path in &report.removed {
            eprintln!("  - {path}");
        }
    }
    if !report.skipped.is_empty() {
        eprintln!();
        eprintln!("{}", t("保留项：", "Kept:"));
        for path in &report.skipped {
            eprintln!("  - {path}");
        }
    }
    eprintln!();
    eprintln!(
        "{}",
        t(
            "请在 Zed → Extensions 中手动禁用或卸载「iOS-Runner」扩展。",
            "In Zed → Extensions, disable or uninstall the「iOS-Runner」extension manually.",
        )
    );
    Ok(())
}

fn cmd_install_self() -> Result<()> {
    let exe = env::current_exe().context("current executable")?;
    let home = dirs::home_dir().context("home directory")?;
    let dest_dir = home.join(ios_runner_core::INSTALL_DIR);
    std::fs::create_dir_all(&dest_dir).context("create install dir")?;
    let dest = dest_dir.join("ios-runner");
    std::fs::copy(&exe, &dest).context("copy binary")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
    }
    eprintln!("✓ Installed {}", dest.display());
    Ok(())
}

fn cmd_doctor(root: &Path) -> Result<()> {
    let mut ok = true;

    eprintln!("iOS-Runner {}", env!("CARGO_PKG_VERSION"));

    let home = dirs::home_dir().context("home directory")?;
    let bundled_cli = home.join(".ios-runner/bin/ios-runner");
    if bundled_cli.is_file() {
        eprintln!("✓ CLI {}", bundled_cli.display());
    } else if let Ok(exe) = env::current_exe() {
        eprintln!("✓ CLI (current) {}", exe.display());
    } else {
        eprintln!("⚠ CLI not at ~/.ios-runner/bin/ios-runner — reload Zed extension or run install-self");
    }

    if global_zed_tasks_contain_legacy_scripts().unwrap_or(false) {
        eprintln!("⚠ Global Zed tasks use legacy scripts — run: ios-runner install-zed-tasks");
    } else if zed_config_dir()
        .map(|d| d.join("tasks.json").is_file())
        .unwrap_or(false)
    {
        eprintln!("✓ Global Zed tasks");
    } else {
        eprintln!("⚠ No global tasks — run: ios-runner install-zed-tasks");
    }

    for (name, args) in [
        ("xcodebuild", &["-version"][..]),
        ("xcrun", &["--version"][..]),
        ("xcrun", &["simctl", "list", "devices"][..]),
    ] {
        if !command_exists(name, args) {
            eprintln!("✗ {name} not available or failed");
            ok = false;
        } else {
            eprintln!("✓ {name}");
        }
    }

    if command_exists("xcbeautify", &["--version"]) {
        eprintln!("✓ xcbeautify");
    } else {
        eprintln!("⚠ xcbeautify not found (optional, brew install xcbeautify)");
    }

    if root.join("Podfile").is_file() {
        if command_exists("pod", &["--version"]) {
            eprintln!("✓ CocoaPods (pod)");
        } else {
            eprintln!("⚠ Podfile present but `pod` not found");
        }
        if !root.join("Pods").is_dir() {
            eprintln!("⚠ Run `pod install` before building");
        }
    }

    match detect_project(root) {
        Ok(p) => eprintln!("✓ Xcode {}: {}", p.kind_label(), p.path.display()),
        Err(e) => {
            eprintln!("✗ {e}");
            ok = false;
        }
    }

    match RunnerConfig::load(root) {
        Ok(config) => {
            if is_placeholder_destination(&config.destination) {
                eprintln!(
                    "⚠ {}",
                    t(
                        "已保存的 destination 无效 — 请运行 ios-runner configure --run",
                        "Saved destination is invalid — run ios-runner configure --run",
                    )
                );
                ok = false;
            } else {
                eprintln!(
                    "✓ {}",
                    tf(
                        || format!("已配置 scheme={} · {}", config.scheme, config.device_summary()),
                        || format!("Configured scheme={} · {}", config.scheme, config.device_summary()),
                    )
                );
            }
        }
        Err(_) => eprintln!(
            "ℹ {}",
            t(
                "此工程尚无配置 — 运行 ios-runner ensure",
                "No config for this project — run ios-runner ensure",
            )
        ),
    }

    if ok {
        eprintln!("\nNext: ios-runner configure   (pick scheme & simulator)");
        eprintln!("      ios-runner init --pick");
    } else {
        bail!("doctor found issues");
    }
    Ok(())
}

fn cmd_init(root: &Path, pick: bool) -> Result<()> {
    let config = if pick {
        configure_project(root, None)?
    } else {
        ensure_project(root)?;
        RunnerConfig::load(root)?
    };

    let project = detect_project(root)?;
    print_config_summary(&config, project.has_podfile);
    print_keybind_hint();
    Ok(())
}

fn cmd_init_ensure(root: &Path, quiet: bool) -> Result<()> {
    let report = ensure_project(root)?;
    if let Ok(config) = RunnerConfig::load(root) {
        config.apply_locale();
    }
    if quiet && !report.wrote_config && !report.wrote_tasks {
        return Ok(());
    }
    if report.wrote_config || report.wrote_tasks {
        eprintln!(
            "{}",
            t(
                "iOS-Runner configured for this project.",
                "iOS-Runner configured for this project.",
            )
        );
    } else {
        eprintln!(
            "{}",
            t(
                "iOS-Runner: already configured. Use `ios-runner configure` to change scheme/simulator.",
                "iOS-Runner: already configured. Use `ios-runner configure` to change scheme/device.",
            )
        );
    }
    eprintln!("  scheme: {}", report.scheme);
    eprintln!("  dest:   {}", report.destination);
    eprintln!(
        "  {}: {}",
        t("全局配置", "Global config"),
        report.global_config.display()
    );
    Ok(())
}

fn cmd_configure(root: &Path, run: bool, no_run: bool) -> Result<()> {
    let run_after = if run {
        Some(true)
    } else if no_run {
        Some(false)
    } else {
        None
    };
    configure_project(root, run_after)?;
    Ok(())
}

fn print_config_summary(config: &RunnerConfig, has_podfile: bool) {
    if let Ok(path) = ios_runner_core::config_file_path() {
        eprintln!(
            "  {}: {}",
            t("全局配置", "Global config"),
            path.display()
        );
    }
    eprintln!("  scheme: {}", config.scheme);
    eprintln!("  path:   {}", config.path);
    eprintln!("  dest:   {}", config.destination);
    if has_podfile {
        eprintln!("\n  CocoaPods: run task「iOS-Runner: Pod Install」if needed");
    }
}

fn print_keybind_hint() {
    eprintln!("\nZed keymap example:");
    eprintln!(r#"  "cmd-b": ["task::Spawn", {{ "task_name": "iOS-Runner: Build" }}],"#);
    eprintln!(r#"  "cmd-r": ["task::Spawn", {{ "task_name": "iOS-Runner: Run" }}]"#);
}

fn cmd_list(root: &Path, what: &str) -> Result<()> {
    match what {
        "schemes" => {
            let project = detect_project(root)?;
            let schemes = list_schemes(root, &project)?;
            println!("{}", serde_json::to_string_pretty(&schemes)?);
        }
        "simulators" => {
            let sims = list_simulators()?;
            println!("{}", serde_json::to_string_pretty(&sims)?);
        }
        "destinations" => {
            let project = detect_project(root)?;
            let scheme = if let Ok(c) = RunnerConfig::load(root) {
                c.scheme
            } else {
                let schemes = list_schemes(root, &project)?;
                schemes
                    .into_iter()
                    .find(|s| !s.starts_with("Pods-"))
                    .context("no scheme")?
            };
            let dests = list_run_destinations(root, &project, &scheme)?;
            println!("{}", serde_json::to_string_pretty(&dests)?);
        }
        _ => bail!("unknown list target: {what} (try: schemes, simulators, destinations)"),
    }
    Ok(())
}

fn command_exists(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

trait KindLabel {
    fn kind_label(&self) -> &'static str;
}

impl KindLabel for ios_runner_core::DetectedProject {
    fn kind_label(&self) -> &'static str {
        use ios_runner_core::ProjectKind;
        match self.kind {
            ProjectKind::Workspace => "workspace",
            ProjectKind::Project => "project",
        }
    }
}
