use zed_extension_api::{ContextServerId, Extension, Project, Result, process::Command};

struct XcodePilotExtension;

impl Extension for XcodePilotExtension {
    fn new() -> Self {
        Self
    }

    /// Starts the Xcode Pilot MCP server. On connect it auto-detects the Xcode project
    /// and writes `.zed/tasks.json` so the user can run「Xcode Pilot: Run」from the task menu.
    fn context_server_command(
        &mut self,
        _server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        Ok(Command::new("xcode-pilot").args(["mcp"]))
    }
}

zed_extension_api::register_extension!(XcodePilotExtension);
