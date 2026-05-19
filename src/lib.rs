use zed_extension_api::{ContextServerId, Extension, Project, Result, process::Command};

struct IosRunnerExtension;

impl Extension for IosRunnerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        Ok(Command::new("ios-runner").args(["mcp"]))
    }
}

zed_extension_api::register_extension!(IosRunnerExtension);
