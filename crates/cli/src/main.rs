use std::env;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use ios_runner_core::{
    RunnerConfig, build_project, configure_project, detect_project, ensure_project, list_schemes,
    list_simulators, resolve_packages, run_on_simulator,
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
    /// Interactive scheme + simulator selection, then save config
    Configure,
    Mcp,
    Build,
    Run,
    ResolvePackages,
    List {
        #[arg(long, default_value = "schemes")]
        what: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = workspace_root()?;

    match cli.command {
        Commands::Doctor => cmd_doctor(&root),
        Commands::Init { pick } => cmd_init(&root, pick),
        Commands::Ensure => cmd_init_ensure(&root),
        Commands::Configure => cmd_configure(&root),
        Commands::Mcp => mcp::run_mcp(),
        Commands::Build => {
            let config = RunnerConfig::load(&root)?;
            config.validate(&root)?;
            build_project(&root, &config)
        }
        Commands::Run => {
            let config = RunnerConfig::load(&root)?;
            config.validate(&root)?;
            run_on_simulator(&root, &config)
        }
        Commands::ResolvePackages => {
            let config = RunnerConfig::load(&root)?;
            config.validate(&root)?;
            resolve_packages(&root, &config)
        }
        Commands::List { what } => cmd_list(&root, &what),
    }
}

fn workspace_root() -> Result<PathBuf> {
    env::current_dir().context("current directory")
}

fn cmd_doctor(root: &PathBuf) -> Result<()> {
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

fn cmd_init(root: &PathBuf, pick: bool) -> Result<()> {
    let config = if pick {
        configure_project(root)?
    } else {
        ensure_project(root)?;
        RunnerConfig::load(root)?
    };

    let project = detect_project(root)?;
    print_config_summary(&config, project.has_podfile);
    print_keybind_hint();
    Ok(())
}

fn cmd_init_ensure(root: &PathBuf) -> Result<()> {
    let report = ensure_project(root)?;
    if report.wrote_config || report.wrote_tasks {
        eprintln!("iOS-Runner configured for this project.");
    } else {
        eprintln!("iOS-Runner: already configured. Use `ios-runner configure` to change scheme/simulator.");
    }
    eprintln!("  scheme: {}", report.scheme);
    eprintln!("  dest:   {}", report.destination);
    Ok(())
}

fn cmd_configure(root: &PathBuf) -> Result<()> {
    let config = configure_project(root)?;
    let project = detect_project(root)?;
    eprintln!("\niOS-Runner configured.");
    print_config_summary(&config, project.has_podfile);
    print_keybind_hint();
    Ok(())
}

fn print_config_summary(config: &RunnerConfig, has_podfile: bool) {
    eprintln!("  Wrote {}", RunnerConfig::FILE_NAME);
    eprintln!("  Wrote .zed/tasks.json");
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

fn cmd_list(root: &PathBuf, what: &str) -> Result<()> {
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
        _ => bail!("unknown list target: {what} (try: schemes, simulators)"),
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
