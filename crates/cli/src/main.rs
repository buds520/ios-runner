use std::env;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use xcode_pilot_core::{
    PilotConfig, build_project, detect_project, ensure_project, list_schemes, resolve_packages,
    run_on_simulator,
};

mod mcp;

#[derive(Parser)]
#[command(name = "xcode-pilot", about = "Build and run Xcode projects for Zed")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check Xcode toolchain and project prerequisites
    Doctor,
    /// Detect project, write .xcode-pilot.toml and .zed/tasks.json
    Init,
    /// Same as init but skip if already configured (used by Zed extension)
    Ensure,
    /// MCP stdio server for Zed (auto-detect project on start)
    Mcp,
    /// Compile for the configured destination
    Build,
    /// Build, install on simulator, and launch
    Run,
    /// Resolve Swift Package dependencies
    ResolvePackages,
    /// List schemes (JSON)
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
        Commands::Init => cmd_init(&root, false),
        Commands::Ensure => cmd_init(&root, true),
        Commands::Mcp => mcp::run_mcp(),
        Commands::Build => {
            let config = PilotConfig::load(&root)?;
            config.validate(&root)?;
            build_project(&root, &config)
        }
        Commands::Run => {
            let config = PilotConfig::load(&root)?;
            config.validate(&root)?;
            run_on_simulator(&root, &config)
        }
        Commands::ResolvePackages => {
            let config = PilotConfig::load(&root)?;
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

    if root.join("Podfile").is_file() {
        if command_exists("pod", &["--version"]) {
            eprintln!("✓ CocoaPods (pod)");
        } else {
            eprintln!("⚠ Podfile present but `pod` not found — install CocoaPods for `pod install`");
        }
        if !root.join("Pods").is_dir() {
            eprintln!("⚠ Run `pod install` before building (Pods/ missing)");
        }
    }

    match detect_project(root) {
        Ok(p) => eprintln!("✓ Xcode project: {}", p.path.display()),
        Err(e) => {
            eprintln!("✗ {e}");
            ok = false;
        }
    }

    if ok {
        eprintln!("\nReady. Run `xcode-pilot init` in this directory.");
    } else {
        bail!("doctor found issues");
    }
    Ok(())
}

fn cmd_init(root: &PathBuf, ensure_only: bool) -> Result<()> {
    if ensure_only {
        let report = ensure_project(root)?;
        if report.wrote_config || report.wrote_tasks {
            eprintln!("Xcode Pilot configured for this project.");
        } else {
            eprintln!("Xcode Pilot: project already configured.");
        }
        eprintln!("  scheme: {}", report.scheme);
        eprintln!("  dest:   {}", report.destination);
        return Ok(());
    }

    let _ = ensure_project(root)?;
    let config = PilotConfig::load(root)?;
    let project = detect_project(root)?;

    eprintln!("Wrote {}", PilotConfig::FILE_NAME);
    eprintln!("Wrote .zed/tasks.json");
    eprintln!("  scheme: {}", config.scheme);
    eprintln!("  path:   {}", config.path);
    eprintln!("  dest:   {}", config.destination);

    if project.has_podfile && !root.join("Pods").is_dir() {
        eprintln!("\nNext: run task「Xcode Pilot: Pod Install」or `pod install`");
    }

    print_keybind_hint();
    Ok(())
}

fn print_keybind_hint() {
    eprintln!("\nBind keys in Zed (example):");
    eprintln!(r#"  "cmd-b": ["task::Spawn", {{ "task_name": "Xcode Pilot: Build" }}],"#);
    eprintln!(r#"  "cmd-r": ["task::Spawn", {{ "task_name": "Xcode Pilot: Run" }}]"#);
}

fn cmd_list(root: &PathBuf, what: &str) -> Result<()> {
    let project = detect_project(root)?;
    match what {
        "schemes" => {
            let schemes = list_schemes(root, &project)?;
            println!("{}", serde_json::to_string_pretty(&schemes)?);
        }
        _ => bail!("unknown list target: {what} (try: schemes)"),
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
