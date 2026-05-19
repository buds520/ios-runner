# iOS-Runner 用户上手（3 分钟）

面向：**已有 iOS / Xcode 工程**，用 **Zed** 打开，想 **编译** 或 **在模拟器里运行**。

---

## 你需要准备什么

| 项 | 说明 |
|----|------|
| 电脑 | **macOS**（必须） |
| Xcode | 从 App Store 安装，并至少打开过一次 |
| Zed | [zed.dev](https://zed.dev/) |
| iOS-Runner CLI | 目前扩展会调用命令行工具，需 **安装一次**（见下） |

> 上架市场后仍是：扩展 + 一条命令安装 CLI。后续版本会做到「只装扩展、不装 Rust」。

---

## 第一次使用（推荐顺序）

### ① 安装 CLI（整台 Mac 只需一次）

在终端执行：

```bash
cargo install ios-runner --git https://github.com/buds520/ios-runner --locked
```

若未安装 Rust，可先安装 [rustup](https://rustup.rs/)，或等扩展 v0.2 提供预编译二进制。

检查：

```bash
ios-runner doctor
```

应看到 `xcodebuild`、`simctl` 等为 ✓。

### ② 在 Zed 安装扩展

1. 打开 Zed  
2. **Extensions**（扩展）  
3. 搜索 **iOS-Runner**  
4. 点击 **Install**

### ③ 用 Zed 打开工程文件夹

**File → Open Folder**，选择包含下面之一的目录（不要只打开单个 `.xcodeproj` 文件）：

- `YourApp.xcworkspace` 所在目录（**CocoaPods 工程用这个**）
- 或 `YourApp.xcodeproj` 所在目录

**CocoaPods 项目**：先在终端进该目录执行一次：

```bash
pod install
```

### ④ 启用 MCP（让扩展自动帮你生成配置）

1. Zed **Settings** → 搜索 **MCP**  
2. 找到 **iOS-Runner**，**打开**  
3. 重新打开该工程文件夹（或重启 Zed）

首次连接时，扩展会运行 `ios-runner ensure`，自动生成：

- `.ios-runner.toml`（scheme、模拟器等）
- `.zed/tasks.json`（编译 / 运行任务）

### ⑤ 选 Scheme 和模拟器（可选但推荐）

自动检测不一定是你想要的 App / 模拟器。在工程目录终端执行：

```bash
ios-runner configure
```

按提示输入编号，选择 **Scheme** 和 **模拟器**。

或在 Zed：**Tasks** → **iOS-Runner: Configure**（会打开终端让你输入编号）。

---

## 日常：编译 / 运行

打开工程后，任选一种方式：

| 操作 | 做法 |
|------|------|
| **运行**（编译 + 装到模拟器 + 启动） | `Cmd+Shift+P` → 输入 `task spawn` → 选 **iOS-Runner: Run** |
| **只编译** | 同上，选 **iOS-Runner: Build** |
| 终端 | 在工程目录：`ios-runner run` / `ios-runner build` |

### 快捷键（可选，配一次）

Zed → **Settings** → 打开 `settings.json`，在 `bindings` 里加：

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-b": ["task::Spawn", { "task_name": "iOS-Runner: Build" }],
      "cmd-r": ["task::Spawn", { "task_name": "iOS-Runner: Run" }]
    }
  }
]
```

之后：**Cmd+B** 编译，**Cmd+R** 运行。

---

## 流程一张图

```
安装 CLI（一次）
    ↓
Zed 市场安装「iOS-Runner」
    ↓
打开 iOS 工程文件夹
    ↓
Settings → 打开 MCP「iOS-Runner」  →  自动生成配置与任务
    ↓
（推荐）ios-runner configure  →  选 Scheme / 模拟器
    ↓
任务「iOS-Runner: Run」或 Cmd+R
```

---

## 常见问题

**任务里只有报错 `ios-runner: command not found`**  
→ CLI 未安装或不在 PATH。重新执行上面的 `cargo install`，并确保 `~/.cargo/bin` 在 PATH 中。

**CocoaPods 工程编译失败**  
→ 是否已 `pod install`？是否打开的是含 `.xcworkspace` 的**目录**？

**模拟器找不到**  
→ 运行 `ios-runner configure` 换一个模拟器；或 Xcode → Settings → Platforms 安装 iOS 模拟器。

**想改 Scheme**  
→ `ios-runner configure`，或编辑 `.ios-runner.toml` 里的 `scheme`、`destination`。

**不想用 MCP**  
→ 在工程目录手动执行：`ios-runner init --pick`（交互配置）或 `ios-runner init`（全自动）。

---

## 和 SweetPad 的对应关系

| SweetPad (VS Code) | iOS-Runner (Zed) |
|------------------|------------------|
| 侧边栏 ▶️ Run | 任务 **iOS-Runner: Run** 或 Cmd+R |
| 选 Scheme / 模拟器 | `ios-runner configure` |
| 安装扩展 | Zed 市场 **iOS-Runner** + 一次性安装 CLI |
