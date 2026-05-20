# iOS-Runner

**English** · **[简体中文](README.zh-CN.md)**

Build and run **your own** iOS Xcode projects from [Zed](https://zed.dev/) (`xcodebuild` + Simulator / device).

**Requirements:** macOS · Xcode · [Zed](https://zed.dev/)

---

## Quick start

```
Install extension → Open Folder (your project) → Cmd+Shift+R to run
```

| Step | What to do |
| ---- | ---------- |
| 1 | Install **extension + CLI** using one of the two methods below |
| 2 | Zed: **File → Open Folder** → directory that contains `.xcodeproj` or `.xcworkspace` |
| 3 | **Cmd+Shift+R** to run (first time: **Cmd+Shift+E** to set up) |

CocoaPods: run `pod install` first, then Open Folder at the **`.xcworkspace`** directory.

---

## Install methods

| Method | For |
| ------ | --- |
| **Method 1: Zed Extensions marketplace** | **iOS Runner** is available in the marketplace |
| **Method 2: Local dev extension** | Not listed yet, or you want the latest code |

| Piece | Role |
| ----- | ---- |
| **Zed extension** | Tasks in the panel (Run / Build / Setup), shortcuts (Cmd+Shift+R, …) |
| **ios-runner CLI** | Runs `xcodebuild`, picks scheme & destination, installed to `~/.ios-runner/bin` |

---

## Method 1: Zed Extensions marketplace

1. Zed → **Cmd+Shift+P** → type `extensions` → Enter
2. Search **iOS Runner** → **Install**
3. Wait a few seconds (CLI is copied to `~/.ios-runner/bin`)
4. **File → Open Folder** → your iOS project directory
5. **Cmd+Shift+R** to run

No need to clone this repo or run `cargo install`.

---

## Method 2: Local dev extension

Run in Terminal ([Rust](https://rustup.rs/) required):

```bash
git clone https://github.com/buds520/ios-runner.git && cd ios-runner && ./install-dev.sh
```

This builds the CLI and writes `~/.config/zed/tasks.json` + keymap.

Then in Zed:

1. **Extensions** → **Install Dev Extension**
2. Select the cloned directory (contains `extension.toml`)
3. **Cmd+Q** to quit Zed completely, then reopen
4. **File → Open Folder** → your iOS project
5. **Cmd+Shift+E** to set up, or **Cmd+Shift+R** to run

---

## After install

| Goal | Action |
| ---- | ------ |
| First time on this project | **Cmd+Shift+E** (interactive scheme & destination) |
| Build and run | **Cmd+Shift+R** |
| Build only | **Cmd+Shift+B** |
| Change simulator / device | **Cmd+Shift+I**, or `ios-runner switch` in Terminal |
| All tasks | **Opt+Shift+T** → search `iOS-Runner` |

Settings live in **`~/.config/ios-runner/config.toml`**, keyed by `.xcodeproj` / `.xcworkspace` path — not committed to your repo by default.

---

## FAQ

**Task panel shows "No matches"**  
→ Ensure **Open Folder** on the project directory (not a single file), then re-run `./install-dev.sh`.

**Duplicate Run tasks**  
→ Remove `<project>/.zed/tasks.json`, run `ios-runner ensure --quiet`.

**Skip rebuild when sources unchanged (optional)**  
→ `IOS_RUNNER_SKIP_IF_FRESH=1 ios-runner run`

**Uninstall**  
→ `ios-runner uninstall`, then disable the extension in Zed **Extensions**.

More: [docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md) · [docs/ZED_UX.md](docs/ZED_UX.md)

---

## License

MIT
