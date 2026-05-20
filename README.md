# iOS-Runner

**English** · [**简体中文**](README.zh-CN.md)

Build and run **your own** iOS Xcode projects from [Zed](https://zed.dev/) (`xcodebuild` + Simulator / device).

**Requirements:** macOS · Xcode · [Zed](https://zed.dev/)

---

## Quick start

```
Install extension → Open Folder (your project) → Cmd+Shift+R to run
```

| Step | What to do |
|------|------------|
| 1 | Install **extension + CLI** using one of the methods below |
| 2 | Zed: **File → Open Folder** → directory that contains `.xcodeproj` or `.xcworkspace` |
| 3 | **Cmd+Shift+R** to run (first time: **Cmd+Shift+E** to set up) |

CocoaPods: run `pod install` first, then Open Folder at the **`.xcworkspace`** directory.

---

## Which install method?

| Your situation | Use | Terminal needed? |
|----------------|-----|------------------|
| **iOS Runner** appears in Zed Extensions | **Method 1** Marketplace | Usually no |
| Not in marketplace yet / want latest code | **Method 2** Dev Extension | One command for CLI |
| Extension installed but task list is empty | **Method 3** CLI only | One command |

> **Marketplace:** under review — [PR #6145](https://github.com/zed-industries/extensions/pull/6145). **Use Method 2 for now.**

### Extension vs CLI

| Piece | Role |
|-------|------|
| **Zed extension** | Tasks in the panel (Run / Build / Setup), shortcuts (Cmd+Shift+R, …) |
| **`ios-runner` CLI** | Runs `xcodebuild`, picks scheme & destination, installed to `~/.ios-runner/bin` |

- **Method 1:** Installing the marketplace extension also installs the CLI automatically — no `cargo`, no clone.
- **Method 2:** Dev Extension does not ship a prebuilt CLI — run **Step A** below before loading the extension in Zed.

---

## Method 1: Zed Extensions marketplace (after listing)

For users who can search and install **iOS Runner** in Zed.

1. Zed → **Cmd+Shift+P** → type `extensions` → Enter  
2. Search **iOS Runner** → **Install**  
3. Wait a few seconds (CLI is copied to `~/.ios-runner/bin`)  
4. **File → Open Folder** → your iOS project directory  
5. **Cmd+Shift+R** to run  

No need to clone this repo or run `cargo install`.

---

## Method 2: Dev Extension (recommended today)

For use before marketplace listing or when testing latest `main`.

**Two steps: install CLI first, then load the extension in Zed.**

### Step A — CLI + global Zed tasks (once, in Terminal)

No clone required:

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/install-dev.sh | bash
```

This clones/updates under `~/.ios-runner/src/ios-runner`, builds the CLI, and writes `~/.config/zed/tasks.json` + keymap.

If you already cloned the repo:

```bash
./install-dev.sh
```

### Step B — Install Dev Extension in Zed

1. Zed → **Extensions** → **Install Dev Extension**  
2. Select the **repository root** (contains `extension.toml` and `src/lib.rs`)  
   - After `install-dev.sh`, usually: `~/.ios-runner/src/ios-runner`  
   - Do **not** select the `XcodePilotDemo` subfolder  
3. **Cmd+Q** to quit Zed completely, then reopen  
4. **File → Open Folder** → your iOS project  
5. **Cmd+Shift+E** to set up, or **Cmd+Shift+R** to run  

If CLI is still missing, the extension log will print the `install-dev.sh` command again.

---

## Method 3: CLI only (empty task list)

For when the extension is already installed but **Opt+Shift+T** shows no iOS-Runner tasks.

1. Confirm you used **Open Folder** on the project directory (not a single file)  
2. Run in Terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

3. **Cmd+Q** Zed → reopen → **Open Folder** → **Cmd+Shift+R**

---

## After install

| Goal | Action |
|------|--------|
| First time on this project | **Cmd+Shift+E** (interactive scheme & destination) |
| Build and run | **Cmd+Shift+R** |
| Build only | **Cmd+Shift+B** |
| Change simulator / device | **Cmd+Shift+I**, or `ios-runner switch` in Terminal |
| All tasks | **Opt+Shift+T** → search `iOS-Runner` |

Settings live in **`~/.config/ios-runner/config.toml`**, keyed by `.xcodeproj` / `.xcworkspace` path — not committed to your repo by default.

Your project does **not** need `.zed/tasks.json`; global tasks are in `~/.config/zed/tasks.json`.

---

## FAQ

**Task panel shows “No matches”**  
→ Use [Method 3](#method-3-cli-only-empty-task-list) and ensure **Open Folder** on the project root.

**Duplicate Run tasks**  
→ Remove `<project>/.zed/tasks.json`, run `ios-runner ensure --quiet`.

**Skip rebuild when sources unchanged (optional)**  
→ `IOS_RUNNER_SKIP_IF_FRESH=1 ios-runner run`

**Uninstall**  
→ `ios-runner uninstall`, then disable the extension in Zed **Extensions**.

More: [docs/NEW_USER.md](docs/NEW_USER.md) · [docs/ZED_UX.md](docs/ZED_UX.md)

---

## Maintainers

[XcodePilotDemo/](XcodePilotDemo/) is for repo testing only, not the user onboarding path.  
Dev / release: [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) · [docs/PUBLISHING.md](docs/PUBLISHING.md)

## License

MIT
