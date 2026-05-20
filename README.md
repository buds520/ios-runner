# iOS-Runner

**English** · [**简体中文**](README.zh-CN.md)

Build and run iOS Xcode projects from [Zed](https://zed.dev/) — similar idea to [SweetPad](https://sweetpad.hyzyla.dev/), but for Zed.

Requires **macOS** and **Xcode**.

---

## Install (new users)

1. Zed → **Extensions** → **iOS Runner** → **Install**
2. **Open Folder** → your iOS project (directory with `.xcodeproj` or `.xcworkspace`)
3. **Cmd+Shift+R** to run

If the task list is empty:

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

No need to clone this repository.

### Developers (this repo)

```bash
git clone https://github.com/buds520/ios-runner.git
cd ios-runner && ./scripts/install.sh
```

See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md). Sample app `XcodePilotDemo/` is for testing only.

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
Install the **iOS Runner** extension, **Open Folder** on your project, then:

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

New projects do not ship with `.zed/tasks.json` — tasks come from global `~/.config/zed/tasks.json`.

**Duplicate tasks in the spawn menu**  
Delete stale `<project>/.zed/tasks.json` and run `ios-runner ensure --quiet`, or reinstall global tasks with `ios-runner install-zed-tasks`.

**CLI not ready / old task scripts**  
Re-run `install-cli.sh` above, or quit Zed (Cmd+Q) and reinstall the extension.

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

## Demo project (maintainers only)

Minimal sample for testing: [XcodePilotDemo/](XcodePilotDemo/) — not the new-user onboarding path.

---

## Docs

| Doc | Topic |
|-----|--------|
| [docs/NEW_USER.md](docs/NEW_USER.md) | New-user flow & troubleshooting |
| [docs/QUICKSTART.md](docs/QUICKSTART.md) | Step-by-step first run |
| [docs/ZED_UX.md](docs/ZED_UX.md) | Tasks, shortcuts, i18n |
| [docs/PUBLISHING.md](docs/PUBLISHING.md) | Releases & Zed marketplace |
| [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) | Hacking on this repo |

---

## License

MIT
