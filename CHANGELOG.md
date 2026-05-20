# Changelog

## [Unreleased]

## [0.2.3] - 2026-05-20

### Added
- CI workflow: `cargo test` + `clippy` on push/PR (macos-14)
- Unit tests: destination, config, locale, global_store, detect, (10+ total)
- MCP: `inputSchema` + optional `scheme` / `destination` / `configuration` / `verbose`
- `doctor`: CLI path, global tasks health, saved destination validation

### Changed
- README / QUICKSTART / DEVELOPMENT / AGENTS / USER_EXPERIENCE aligned with v0.2.2+ flow
- Zed tasks unified via `tasks::TASK_DEFS`
- `xcbeautify` probe cached (`OnceLock`); single warn when enabled but missing
- Project detection deduplicated (`find_xcode_file`)
- Destination line parser accepts `key=` and `key:` (xcodebuild formats)
- MCP `initialize` no longer runs `install-self` side effect

## [0.2.2] - 2026-05-19

### Added
- `ios-runner uninstall` — remove CLI, Zed tasks/keymap, and global config (`--keep-config`, `--purge-derived-data`)
- Zed extension ships macOS CLI in `bin/` (offline install to `~/.ios-runner/bin` on load)

### Fixed
- Invalid `destination` for xcodebuild (`key=value` format, reject placeholder simulators)
- `ensure` replaces invalid saved destination with a real default simulator
- Simulator log stream: `exec` `simctl launch --console-pty` so Ctrl+C stops the log (Zed terminal)
- Zed tasks: no `curl` download; use `$HOME` / quoted paths (Zed variable expansion safe)
- Boot simulator when already Booted no longer fails

### Changed
- Extension bootstrap installs bundled CLI once per version; GitHub download only as fallback
- Zed task schema `tasks-v6-quoted-home`; removed legacy project `.zed/tasks.json` from demo

## [0.2.1] - 2026-05-19

### Fixed
- Physical device `showBuildSettings`: use `iphoneos` SDK instead of hardcoded simulator SDK
- Default simulator destination: prefer `simctl` list; no silent fallback to `iPhone 16`
- Global config `config.toml`: file lock on read-modify-write (parallel Zed tasks)
- Stable DerivedData cache folder names (FNV-1a instead of `DefaultHasher`)
- Swift Package resolve failures are warned instead of silently ignored
- Zed task CLI download pinned to release version (not `latest`)
- Extension bootstrap runs once per version; clearer errors
- Tighter devicectl trust/lock hints; `xcbeautify` default aligned with global config

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
