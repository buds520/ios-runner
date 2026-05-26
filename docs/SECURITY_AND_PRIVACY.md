# Security and Privacy

iOS-Runner is a local Zed extension and CLI for building and running Xcode projects.

## Data Handling

iOS-Runner does not collect telemetry, upload source code, or send project data to any external service.

The extension and CLI operate on the local machine using Apple's developer tools:

- `xcodebuild`
- `xcrun simctl`
- `xcrun devicectl`
- `open` for launching macOS apps or Simulator

Network access is only used when Zed starts the MCP server and the matching CLI binary is not already present in the extension work directory. In that case, the extension downloads the release asset from the `buds520/ios-runner` GitHub Releases page.

## Local Files Written

The extension or CLI may write these files:

| Path | Purpose |
| ---- | ------- |
| `bin/ios-runner-*` in the extension work directory | MCP server binary downloaded by the Zed extension |
| `~/.ios-runner/bin/ios-runner` | Optional CLI binary used by manually installed Zed tasks |
| `~/.ios-runner/DerivedData/` | Per-project build cache |
| `~/.config/ios-runner/config.toml` | Scheme, destination, and user preferences |
| `~/.config/zed/tasks.json` | Global Zed tasks for Run, Build, Doctor, and setup |
| `~/.config/zed/keymap.json` | Optional key bindings for iOS-Runner tasks |
| `<project>/.zed/tasks.json` | Project-local helper tasks such as Pod Install and verbose build |
| `<project>/.ios-runner.toml` | Optional local config only when `IOS_RUNNER_LOCAL_CONFIG=1` |

## MCP Server Binary

Marketplace releases do not rely on a bundled CLI checked into the extension repository. When Zed needs to start the MCP server, the extension downloads the matching macOS release asset on demand:

- `ios-runner-aarch64-apple-darwin`
- `ios-runner-x86_64-apple-darwin`

The extension does not write Zed task or keymap files on load. Users who want keyboard shortcuts can install the CLI separately and run `ios-runner install-zed-tasks`.

## Project Access

iOS-Runner reads the currently opened project directory to detect:

- `.xcodeproj`
- `.xcworkspace`
- `Podfile`
- shared schemes and available run destinations reported by Xcode

Builds and runs happen through local Xcode tooling. The extension does not implement its own compiler, debugger, package resolver, or code signing service.

## Uninstall

Run:

```bash
~/.ios-runner/bin/ios-runner uninstall
```

Use optional flags when needed:

```bash
~/.ios-runner/bin/ios-runner uninstall --keep-config
~/.ios-runner/bin/ios-runner uninstall --purge-derived-data
```

Then disable or uninstall the Zed extension from Zed's Extensions panel.
