# iOS-Runner 用户上手（3 分钟）

面向：**已有 iOS / Xcode 工程**，用 **Zed** 打开，想 **编译** 或 **在模拟器里运行**。

---

## 你需要准备什么

| 项 | 说明 |
|----|------|
| 电脑 | **macOS**（必须） |
| Xcode | 从 App Store 安装，并至少打开过一次 |
| Zed | [zed.dev](https://zed.dev/) |
| iOS-Runner | Zed 扩展（内置 CLI，**一般不需要** Rust / cargo） |

---

## 第一次使用（推荐顺序）

### ① 安装 Zed 扩展

1. Zed → **Extensions** → 搜索 **iOS Runner** → **Install**
2. **重载扩展**（扩展会把 CLI 装到 `~/.ios-runner/bin/`）

### ② 写入全局任务（整台 Mac 一次）

```bash
ios-runner install-zed-tasks
```

会写入 `~/.config/zed/tasks.json` 和快捷键（Cmd+Shift+E / I / R）。

检查环境：

```bash
ios-runner doctor
```

应看到 `xcodebuild`、`simctl` 等为 ✓。

> 没有 `ios-runner` 命令？扩展可能尚未 bootstrap：重载扩展，或 [Releases](https://github.com/buds520/ios-runner/releases) 下载二进制 / 开发者 `cargo install --path crates/cli`。

### ③ 用 Zed 打开工程文件夹

**File → Open Folder**，选择包含 `.xcworkspace` 或 `.xcodeproj` 的**目录**（不要只打开单个工程文件）。

**CocoaPods**：先在该目录执行 `pod install`。

### ④ 初始化 + 选设备

- **Cmd+Shift+E** 或任务 **iOS-Runner: Setup Project**（首次）
- **Cmd+Shift+I** 或 `ios-runner configure --run`（选 Scheme / 模拟器 / 真机）

配置默认保存在 `~/.config/ios-runner/config.toml`，**不会**改你的 git 仓库（除非 `IOS_RUNNER_LOCAL_CONFIG=1`）。

### ⑤ 运行

- **Cmd+Shift+R** 或任务 **iOS-Runner: Run**
- 终端：`ios-runner run`

---

## 日常命令

| 操作 | 做法 |
|------|------|
| 运行（编译 + 安装 + 启动） | **Cmd+Shift+R** / **iOS-Runner: Run** |
| 只编译 | **iOS-Runner: Build** / `ios-runner build` |
| 换 Scheme / 设备 | **Cmd+Shift+I** / `ios-runner configure` |
| 详细编译日志 | **iOS-Runner: Build (verbose)** 或 `IOS_RUNNER_RAW_LOG=1 ios-runner build` |

---

## 流程一张图

```
Zed 安装 iOS-Runner 扩展 → 重载
    ↓
ios-runner install-zed-tasks（一次）
    ↓
Open Folder 打开 iOS 工程
    ↓
Cmd+Shift+E 初始化 → Cmd+Shift+I 选设备（推荐）
    ↓
Cmd+Shift+R 运行
```

---

## 常见问题

**Run 面板 No matches**  
→ 执行 `ios-runner install-zed-tasks`。新工程没有 `.zed/tasks.json` 是正常的。

**ios-runner: command not found**  
→ 重载扩展；或 `cargo install --path crates/cli`；确认 `~/.ios-runner/bin` 在 PATH。

**destination 无效 / xcodebuild 失败**  
→ `ios-runner configure --run` 重新选择模拟器或真机。

**CocoaPods 编译失败**  
→ 是否已 `pod install`？是否打开含 `.xcworkspace` 的目录？

**真机**  
→ 解锁、信任、开发者模式；在 Xcode 或 `config.toml` 配置签名 Team。

**改终端语言**  
→ `config.toml` 里 `[defaults] language = "en"` 或 `export IOS_RUNNER_LANG=en`。

---

## 和 SweetPad 的对应

| SweetPad (VS Code) | iOS-Runner (Zed) |
|--------------------|------------------|
| 侧边栏 Run | **iOS-Runner: Run** / Cmd+Shift+R |
| 选 Scheme / 设备 | `configure` / Cmd+Shift+I |
| 装扩展 | Zed **iOS-Runner** + `install-zed-tasks` |

更多：[README.zh-CN.md](../README.zh-CN.md) · [ZED_UX.md](ZED_UX.md)
