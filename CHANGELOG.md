# Changelog

## [0.2.0] - 2026-05-20

### Added
- Global config at `~/.config/ios-runner/config.toml` (default: no project-local `.ios-runner.toml`)
- Global Zed tasks + keymaps (`install-zed-tasks`)
- Terminal UI language: `language` / `IOS_RUNNER_LANG` (zh-CN / en)
- Device lock / trust / Developer Mode hints for physical devices
- Extension bootstrap: download CLI + install global tasks on load

### Changed
- DerivedData under `~/.ios-runner/DerivedData/` instead of inside projects
- Zed tasks hide misleading `Command: /bin/zsh` summary (`show_command: false`)
- Fix Zed task variable clash (`ir_bin` instead of `$IOS_RUNNER`)

### Fixed
- GitHub Release workflow builds from `crates/` directory
- CocoaPods workspace detection and simulator destinations

## [0.1.0] - 2026-05-19

- Initial public release: CLI, Zed extension, MCP, Build/Run tasks
