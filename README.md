# iOS-Runner

**English** · [**简体中文**](README.zh-CN.md)

Build and run iOS Xcode projects from [Zed](https://zed.dev/). Requires **macOS** and **Xcode**.

---

## Install

### 1. Zed Extensions (recommended)

When listed in the catalog:

1. Zed → **Extensions** → search **iOS Runner** → **Install**
2. **File → Open Folder** → your iOS project (folder with `.xcodeproj` or `.xcworkspace`)

**Marketplace status:** under review — [zed-industries/extensions#6145](https://github.com/zed-industries/extensions/pull/6145). Until merged, use **Dev Extension** (below) or install the CLI only (troubleshooting).

### 2. Dev Extension (before marketplace / latest from source)

```bash
git clone https://github.com/buds520/ios-runner.git
```

1. Zed → **Extensions** → **Install Dev Extension** → select the **repo root** (must contain `extension.toml`)
2. **File → Open Folder** → your iOS project

Optional: build CLI from source — `cd ios-runner/crates && cargo install --path cli --locked`  
Or use the bundled installer: `./scripts/install.sh` (see [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)).

---

## Use (both install paths)

| Step | Action |
|------|--------|
| Open project | **File → Open Folder** on the directory with `.xcodeproj` / `.xcworkspace` (CocoaPods: run `pod install` first) |
| First time | **Cmd+Shift+E** (Setup) or task **iOS-Runner: Setup Project** |
| Run | **Cmd+Shift+R** or task **iOS-Runner: Run** |
| Scheme / device | **Cmd+Shift+I** or **iOS-Runner: Select Scheme & Device** |

| Shortcut | Action |
|----------|--------|
| Cmd+Shift+E | Setup / ensure project |
| Cmd+Shift+I | Select scheme & device |
| Cmd+Shift+R | Build & run |

Your project does **not** need `.zed/tasks.json` upfront — the extension writes global tasks under `~/.config/zed/tasks.json`.

---

## If the task list is empty

1. Confirm you used **Open Folder** on the project root (not a single file).
2. Install or refresh the CLI (no clone required):

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

3. Quit Zed (**Cmd+Q**) and reopen, or reinstall the extension.

---

## Config paths

| Path | Purpose |
|------|---------|
| `~/.config/ios-runner/config.toml` | Scheme, destination, defaults |
| `~/.ios-runner/bin/ios-runner` | CLI (extension bootstrap or `install-cli.sh`) |
| `~/.config/zed/tasks.json` | Global Zed tasks |

`IOS_RUNNER_LOCAL_CONFIG=1` also writes `.ios-runner.toml` in the project.

---

## Troubleshooting

**Duplicate tasks** — remove stale `<project>/.zed/tasks.json`, run `ios-runner ensure --quiet`.

**xcodebuild exit 64** — `ios-runner configure --run` and pick a valid simulator or device.

**Uninstall** — `ios-runner uninstall` (add `--keep-config` / `--purge-derived-data` as needed); disable the extension in Zed.

More: [docs/NEW_USER.md](docs/NEW_USER.md) · [docs/ZED_UX.md](docs/ZED_UX.md)

---

## Maintainers

Sample app [XcodePilotDemo/](XcodePilotDemo/) is for testing only. See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) · [docs/PUBLISHING.md](docs/PUBLISHING.md).

## License

MIT
