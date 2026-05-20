# iOS-Runner

**English** · [**简体中文**](README.zh-CN.md)

Build and run iOS Xcode projects from [Zed](https://zed.dev/) — similar idea to [SweetPad](https://sweetpad.hyzyla.dev/), but for Zed.

Requires **macOS** and **Xcode**.

---

## Install

### Option A — Zed extension (recommended)

1. Zed → **Extensions** → search **iOS-Runner** → **Install**
2. Reload Zed once. The extension copies a bundled CLI to `~/.ios-runner/bin/ios-runner` (no Rust or `curl` needed).
3. Install global tasks (once per Mac):

```bash
ios-runner install-zed-tasks
```

### Option B — CLI only (developers)

```bash
cd crates && cargo install --path cli --locked
ios-runner install-zed-tasks
```

Or download a binary from [Releases](https://github.com/buds520/ios-runner/releases).

---

## Quick start

1. **File → Open Folder** on the directory that contains your `.xcworkspace` or `.xcodeproj`  
   (CocoaPods: run `pod install` first)
2. First time: **Cmd+Shift+E** (Setup) or task **iOS-Runner: Setup Project**
3. **Cmd+Shift+R** (Run) or task **iOS-Runner: Run**

Pick scheme and simulator/device: **Cmd+Shift+I** or **iOS-Runner: Select Scheme & Device**.

| Shortcut | Action |
|----------|--------|
| Cmd+Shift+E | Setup / ensure project |
| Cmd+Shift+I | Select scheme & device |
| Cmd+Shift+R | Build & run |

Details: [docs/ZED_UX.md](docs/ZED_UX.md) · [docs/QUICKSTART.md](docs/QUICKSTART.md)

---

## Where settings live

| Path | Purpose |
|------|---------|
| `~/.config/ios-runner/config.toml` | Scheme, destination, defaults (default — **not** in your repo) |
| `~/.ios-runner/bin/ios-runner` | CLI installed by extension or `install-self` |
| `~/.config/zed/tasks.json` | Global Zed tasks (`install-zed-tasks`) |

Set `IOS_RUNNER_LOCAL_CONFIG=1` to also write `.ios-runner.toml` inside the project.

---

## Troubleshooting

**Run panel shows “No matches”**  
Run `ios-runner install-zed-tasks` once. New projects do not ship with `.zed/tasks.json` by default.

**Still see “Downloading CLI…” or old task scripts**  
Reload the extension, run `ios-runner install-zed-tasks`, remove any legacy `.zed/tasks.json` in the project.

**Invalid destination / xcodebuild exit 64**  
Run `ios-runner configure --run` and pick a real simulator or device.

**Physical device**  
Unlock the phone, trust the Mac, enable Developer Mode. Errors include Chinese/English hints when possible.

**Terminal language**  
`language = "en"` in `[defaults]` inside `config.toml`, or `export IOS_RUNNER_LANG=en`.

**Uninstall**

```bash
ios-runner uninstall                      # CLI, Zed tasks/keymap, global config
ios-runner uninstall --keep-config        # keep ~/.config/ios-runner/
ios-runner uninstall --purge-derived-data # also remove build cache
```

Disable the Zed extension manually in **Extensions**.

---

## Demo project

Minimal SwiftUI sample: [XcodePilotDemo/](XcodePilotDemo/)

---

## Docs

| Doc | Topic |
|-----|--------|
| [docs/QUICKSTART.md](docs/QUICKSTART.md) | Step-by-step first run |
| [docs/ZED_UX.md](docs/ZED_UX.md) | Tasks, shortcuts, i18n |
| [docs/PUBLISHING.md](docs/PUBLISHING.md) | Releases & Zed marketplace |
| [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) | Hacking on this repo |

---

## License

MIT
