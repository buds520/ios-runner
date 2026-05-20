# iOS-Runner

**English** · **[简体中文](README.zh-CN.md)**

Build and run **your own** Xcode projects from [Zed](https://zed.dev/) (iOS / iPadOS / macOS — `xcodebuild` + Simulator / device / Mac).

**Requirements:** macOS · Xcode · [Zed](https://zed.dev/)

---

## Quick start

```
Install extension → Open Folder (your app) → Cmd+Shift+R
```

| Shortcut | Action |
| -------- | ------ |
| **Cmd+Shift+R** | Run |
| **Cmd+Shift+B** | Build |
| **Cmd+Shift+I** | Scheme / device (save only, no run) |
| **Cmd+Shift+U** | Set up project |

CocoaPods: run `pod install`, then Open Folder at the **`.xcworkspace`** directory.

---

## Method 1: Marketplace

1. Zed → **Extensions** → search **iOS Runner** → Install
2. **Open Folder** → your app project
3. **Cmd+Shift+R**

No clone, no Rust.

---

## Method 2: Local dev extension

Clone anywhere (e.g. `~/ios-runner`) — **not** inside your app repo.

```bash
git clone https://github.com/buds520/ios-runner.git ~/ios-runner && cd ~/ios-runner && ./install-dev.sh
```

The script installs Rust if needed, builds the CLI, and writes Zed tasks.

| Step | In Zed |
| ---- | ------ |
| 1 | **Install Dev Extension** → select `~/ios-runner` (plugin source) |
| 2 | **Cmd+Q** restart → **Open Folder** → your app project |
| 3 | **Cmd+Shift+U** set up → **Cmd+Shift+R** run |

---

## FAQ

**Task panel “No matches”** → Open Folder on the project root, then re-run `./install-dev.sh`.

**Duplicate tasks** → Remove `<project>/.zed/tasks.json`, run `ios-runner ensure --quiet`.

**macOS app** → Same shortcuts as iOS; after setup the destination shows “My Mac”, and Cmd+Shift+R builds and launches locally.

**Uninstall** → `~/.ios-runner/bin/ios-runner uninstall`, then disable the extension in Zed. (Use the full path if `ios-runner` is not on PATH; re-run `./install-dev.sh` to add it.)

More: [docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md)

---

## License

MIT
