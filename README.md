# iOS-Runner

**English** · **[简体中文](README.zh-CN.md)**

Build and run **your own** Xcode projects from [Zed](https://zed.dev/) (iOS / iPadOS / macOS — `xcodebuild` + Simulator / device / Mac).

Use Zed as a lightweight Xcode launcher: choose a scheme and destination, build, run, and stream app logs without leaving the editor.

**Requirements:** macOS · Xcode · [Zed](https://zed.dev/)

---

## Quick start

```
Install extension → Open your app folder → Cmd+Shift+U → Cmd+Shift+R
```

| Shortcut | Action |
| -------- | ------ |
| **Cmd+Shift+R** | Run |
| **Cmd+Shift+B** | Build |
| **Cmd+Shift+I** | Scheme / device (save only, no run) |
| **Cmd+Shift+U** | Set up project |

Task panel extras: **iOS-Runner: Doctor**, **Pod Install** (CocoaPods), **Build (verbose)**.

### Which folder should I open?

| Project type | Open Folder |
| ------------ | ----------- |
| `.xcodeproj` | The directory that contains the `.xcodeproj` |
| CocoaPods | Run `pod install`, then open the directory that contains the `.xcworkspace` |
| Local dev extension | Install Dev Extension from the `ios-runner` repo, then open your separate app repo |

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

## Troubleshooting

First run **iOS-Runner: Doctor** from Zed's task panel, or:

```bash
ios-runner doctor
```

Common fixes:

| Symptom | Fix |
| ------- | --- |
| Task panel “No matches” | Open your app project folder, then run `ios-runner install-zed-tasks` |
| CocoaPods build fails | Run **iOS-Runner: Pod Install** or `pod install` |
| Device/simulator changed | Press **Cmd+Shift+I** or run `ios-runner switch` |
| Device signing fails | Open the project in Xcode → Target → Signing & Capabilities → select a Team |
| Need full logs | Run **iOS-Runner: Build (verbose)** |

---

## FAQ

**Duplicate tasks** → Remove `<project>/.zed/tasks.json`, run `ios-runner ensure --quiet`.

**macOS app** → Same shortcuts as iOS; after setup the destination shows “My Mac”, and Cmd+Shift+R builds and launches locally.

**Privacy** → iOS-Runner uses local Apple tools and does not upload project data. See [Security and Privacy](docs/SECURITY_AND_PRIVACY.md).

**Uninstall** → `~/.ios-runner/bin/ios-runner uninstall`, then disable the extension in Zed. (Use the full path if `ios-runner` is not on PATH; re-run `./install-dev.sh` to add it.)

More: [docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md)

---

## License

MIT
