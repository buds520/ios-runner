# Changelog

## [Unreleased]

## [0.2.5] - 2026-05-20

### Added
- `scripts/install-cli.sh` — install CLI + global Zed tasks without cloning the repo
- `scripts/install.sh` — install from local clone (Release → bundled `bin/` → cargo fallback)
- `scripts/simulate-fresh-install.sh` — reset user state for new-user UX testing
- `docs/NEW_USER.md` — new-user flow (own iOS project, no Demo shortcut)
- Extension embeds `src/embedded_global_tasks.json` + `embedded_keymap_entry.json` (bootstrap without CLI)
- CLI: `ensure --quiet`, `emit-global-tasks-json`, `emit-embedded-keymap-json`
- Localized global task labels: **初始化项目** / **运行** / **编译** / **选择 Scheme 与设备**

### Fixed
- Empty Run panel: embedded global tasks written on extension load; `${ZED_WORKTREE_ROOT:.}` prevents Zed filtering tasks
- Duplicate tasks in spawn menu: project `.zed/tasks.json` only holds extras (Pod Install, verbose build, …); globals in `~/.config/zed`
- Verbose terminal noise: removed default `IOS_RUNNER_RAW_LOG=1`; `zsh -fc` skips `.zshrc` env dump; `ensure --quiet` on run/build
- Bootstrap error messages: no fake「Reload Window」; use Cmd+Q / reinstall extension
- Extension bootstrap continues when CLI install fails (tasks/keymap still written)

### Changed
- README focused on new users with their own iOS project; `XcodePilotDemo/` marked maintainers-only
- Removed committed `XcodePilotDemo/.ios-runner.toml`
- Zed task schema bumps through `TASKS_SCHEMA` in extension

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

### Changed
- Global config default (not per-repo `.ios-runner.toml`)
- Zed task schema v6+; extension bootstrap once per version

## [0.2.1] - 2026-05-18

### Added
- Global config store (`~/.config/ios-runner/config.toml`)
- Zed global tasks + keymaps (`install-zed-tasks`)

### Changed
- Extension bootstrap: download CLI + install global tasks on load

## [0.2.0] - 2026-05-17

Initial public release.
