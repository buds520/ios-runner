use std::io::{self, BufRead, Write};
use std::path::Path;

use anyhow::{Context, Result};
use ios_runner_core::{
    RunnerConfig, build_project, detect_project, ensure_project, run_app,
};
use serde_json::{json, Value};

pub fn run_mcp() -> Result<()> {
    let root = std::env::current_dir().context("current directory")?;
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let request: Value = serde_json::from_str(&line).context("parse MCP request")?;
        let id = request.get("id").cloned();
        let method = request
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or("");

        let result = match method {
            "initialize" => {
                let setup_msg = auto_setup(&root);
                eprintln!("[ios-runner] {setup_msg}");
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": {
                        "name": "ios-runner",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                })
            }
            "notifications/initialized" => continue,
            "tools/list" => json!({
                "tools": [
                    tool_desc(
                        "ios_runner_detect",
                        "Detect Xcode/CocoaPods project in workspace",
                        json!({ "type": "object", "properties": {} }),
                    ),
                    tool_desc(
                        "ios_runner_setup",
                        "Detect Xcode project and save run settings to ~/.config/ios-runner/config.toml",
                        json!({ "type": "object", "properties": {} }),
                    ),
                    tool_desc(
                        "ios_runner_build",
                        "Build the iOS app with xcodebuild",
                        json!({
                            "type": "object",
                            "properties": {
                                "scheme": { "type": "string", "description": "Override Xcode scheme" },
                                "configuration": { "type": "string", "enum": ["Debug", "Release"] },
                                "destination": { "type": "string", "description": "xcodebuild -destination string" },
                                "verbose": { "type": "boolean", "description": "Set IOS_RUNNER_RAW_LOG=1" }
                            }
                        }),
                    ),
                    tool_desc(
                        "ios_runner_run",
                        "Build, install on simulator/device, and launch",
                        json!({
                            "type": "object",
                            "properties": {
                                "scheme": { "type": "string" },
                                "configuration": { "type": "string", "enum": ["Debug", "Release"] },
                                "destination": { "type": "string" },
                                "verbose": { "type": "boolean" }
                            }
                        }),
                    ),
                ]
            }),
            "tools/call" => handle_tool_call(&root, request.get("params"))?,
            _ => json!({}),
        };

        if method == "notifications/initialized" {
            continue;
        }

        let response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        });
        writeln!(stdout, "{response}")?;
        stdout.flush()?;
    }
    Ok(())
}

fn tool_desc(name: &str, description: &str, input_schema: Value) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema
    })
}

fn apply_tool_args(config: &mut RunnerConfig, args: Option<&Value>) {
    let Some(args) = args else {
        return;
    };
    if let Some(s) = args.get("scheme").and_then(|v| v.as_str()) {
        config.scheme = s.to_string();
    }
    if let Some(d) = args.get("destination").and_then(|v| v.as_str()) {
        config.destination = d.to_string();
    }
    if let Some(c) = args.get("configuration").and_then(|v| v.as_str()) {
        config.configuration = c.to_string();
    }
    if args
        .get("verbose")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        std::env::set_var("IOS_RUNNER_RAW_LOG", "1");
    }
}

fn handle_tool_call(root: &Path, params: Option<&Value>) -> Result<Value> {
    let name = params
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("");
    let arguments = params.and_then(|p| p.get("arguments"));

    let text = match name {
        "ios_runner_detect" => match detect_project(root) {
            Ok(p) => format!(
                "Detected {} ({})",
                p.path.display(),
                if p.has_podfile { "CocoaPods" } else { "Xcode" }
            ),
            Err(e) => format!("Not an Xcode workspace: {e}"),
        },
        "ios_runner_setup" => auto_setup(root),
        "ios_runner_build" => {
            let mut config = RunnerConfig::load(root)?;
            apply_tool_args(&mut config, arguments);
            config.validate(root)?;
            build_project(root, &config)?;
            "Build succeeded.".to_string()
        }
        "ios_runner_run" => {
            let mut config = RunnerConfig::load(root)?;
            apply_tool_args(&mut config, arguments);
            config.validate(root)?;
            run_app(root, &config)?;
            "Run succeeded.".to_string()
        }
        _ => format!("Unknown tool: {name}"),
    };

    Ok(json!({
        "content": [{ "type": "text", "text": text }],
        "isError": false
    }))
}

fn auto_setup(root: &Path) -> String {
    match detect_project(root) {
        Ok(_) => match ensure_project(root) {
            Ok(r) => {
                let mut parts = vec![format!(
                    "iOS-Runner ready: scheme={} destination={}",
                    r.scheme, r.destination
                )];
                if r.wrote_config {
                    parts.push(format!(
                        "saved global config {}",
                        r.global_config.display()
                    ));
                }
                if r.wrote_tasks {
                    parts.push("created .zed/tasks.json (optional)".to_string());
                }
                if r.has_podfile {
                    parts.push("Podfile detected: run「iOS-Runner: Pod Install」if needed".to_string());
                }
                parts.join("; ")
            }
            Err(e) => format!("Setup failed: {e}"),
        },
        Err(_) => "No Xcode project in this folder.".to_string(),
    }
}
