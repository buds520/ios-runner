use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use ios_runner_core::{
    RunnerConfig, build_project, configure_project, detect_project, ensure_project,
    init_locale, install_global_zed_keymap, install_global_zed_tasks, list_run_destinations,
    list_schemes, list_simulators, resolve_packages, run_app, t,
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
    Ensure,
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
    InstallZedTasks,
    List {
        #[arg(long, default_value = "schemes")]
        what: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = workspace_root()?;
    init_locale(Some(&root));

    match cli.command {
        Commands::Doctor => cmd_doctor(&root),
        Commands::Init { pick } => cmd_init(&root, pick),
        Commands::Ensure => cmd_init_ensure(&root),
        Commands::Configure { run, no_run } => cmd_configure(&root, run, no_run),
        Commands::Mcp => mcp::run_mcp(),
        Commands::InstallSelf => cmd_install_self(),
        Commands::InstallZedTasks => cmd_install_zed_tasks(),
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

fn cmd_install_zed_tasks() -> Result<()> {
    let tasks = install_global_zed_tasks()?;
    let keymap = install_global_zed_keymap()?;
    eprintln!(
        "{} {}",
        t("✓ iOS-Runner 任务已写入", "✓ Wrote iOS-Runner tasks to"),
        tasks.display()
    );
    eprintln!(
        "{} {}",
        t("✓ 快捷键已写入", "✓ Wrote keybindings to"),
        keymap.display()
    );
    eprintln!("{}", t("  重启 Zed 后可用：", "  Restart Zed, then use:"));
    eprintln!(
        "{}",
        t(
            "    Cmd+Shift+R  运行   Cmd+Shift+B  编译",
            "    Cmd+Shift+R  Run   Cmd+Shift+B  Build",
        )
    );
    eprintln!(
        "{}",
        t(
            "    Cmd+Shift+I  选设备 Cmd+Shift+E  初始化工程",
            "    Cmd+Shift+I  Device   Cmd+Shift+E  Setup",
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

fn cmd_init_ensure(root: &Path) -> Result<()> {
    let report = ensure_project(root)?;
    if let Ok(config) = RunnerConfig::load(root) {
        config.apply_locale();
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
