# iOS Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 **你自己的 Xcode 项目**（iOS / iPadOS / macOS，`xcodebuild` + 模拟器/真机/Mac）。

把 Zed 当成一个轻量 Xcode 启动器：选择 scheme 和运行目标、编译、运行，并在编辑器里查看 App 日志。

**环境要求：** macOS · Xcode · [Zed](https://zed.dev/)

---

## 快速开始

```
装扩展 → 打开你的 App 项目目录 → 在 Zed Agent 里使用 iOS-Runner 工具
```

| 快捷键 | 动作 |
|--------|------|
| **Cmd+Shift+U** | 初始化项目 |
| **Cmd+Shift+R** | 运行 |
| **Cmd+Shift+B** | 编译 |
| **Cmd+Shift+I** | 选择 Scheme 与运行目标（不运行） |

CLI 写入任务后可用的全局任务：**检查环境**、**初始化项目**、**运行**、**选择 Scheme 与运行目标**、**编译**。
打开或初始化 App 项目后的补充任务：**Pod Install**（CocoaPods）、**编译（详细日志）**、**解析 Swift Packages**、**仅选择（不运行）**。

### 应该打开哪个目录？

| 项目类型 | Open Folder |
|----------|-------------|
| `.xcodeproj` | 包含 `.xcodeproj` 的目录 |
| CocoaPods | 打开 App 项目目录，运行 **iOS-Runner: Pod Install** 或 `pod install` 生成 `.xcworkspace`，再重新初始化项目或运行 |
| 本地开发扩展 | Install Dev Extension 选 `ios-runner` 仓库；Open Folder 选你的 App 仓库 |

---

## 方式一：扩展市场

1. Zed → **Extensions** → 搜索 **iOS Runner** → Install
2. **Open Folder** → 你的 App 项目
3. 打开 Zed Agent，使用 iOS-Runner MCP 工具

Agent/MCP 用法无需 clone、无需 Rust。快捷键和 Run 面板任务由 CLI 执行 `ios-runner install-zed-tasks` 后写入。

---

## 方式二：本地扩展

clone 放哪都行（如 `~/ios-runner`），**不要** clone 进 App 项目里。

```bash
git clone https://github.com/buds520/ios-runner.git ~/ios-runner && cd ~/ios-runner && ./install-dev.sh
```

脚本会自动装 Rust（如需要）、编译 CLI、写入 Zed 任务。

| 步骤 | 在 Zed 里做什么 |
|------|----------------|
| 1 | **Install Dev Extension** → 选 `~/ios-runner`（插件目录） |
| 2 | **Cmd+Q** 重启 → **Open Folder** → 你的 App 项目 |
| 3 | **Cmd+Shift+U** 初始化项目 → **Cmd+Shift+R** 运行 |

---

## 排障流程

先在 Zed 任务面板运行 **iOS-Runner: 检查环境**，或执行：

```bash
ios-runner doctor
```

常见处理：

| 现象 | 处理 |
|------|------|
| 任务面板 No matches | 打开你的 App 项目目录，按 **Cmd+Shift+U**；也可运行 `ios-runner install-zed-tasks`。如果扩展尚未就绪，完全退出并重新打开 Zed，再运行检查环境 |
| CocoaPods workspace 缺失 | 运行 **iOS-Runner: Pod Install** 或 `pod install` 生成 `.xcworkspace`，再重新初始化项目或运行 |
| 运行目标换了或失效 | 按 **Cmd+Shift+I** 或运行 `ios-runner switch` |
| 真机签名失败 | 用 Xcode 打开工程 → Target → Signing & Capabilities → 选择 Team |
| 需要完整日志 | 运行 **iOS-Runner: 编译（详细日志）** |

---

## 常见问题

**重复任务** → 删项目内 `.zed/tasks.json`，执行 `ios-runner ensure --quiet`。

**macOS 应用** → 与 iOS 相同快捷键；初始化后目标会显示「My Mac」，Cmd+Shift+R 编译并在本机启动。

**任务面板开头一堆 `HOME=…`、`ZED_*=…`** → 那是 **Zed 任务终端**在注入工程环境时打印的变量列表（`ZED_ENVIRONMENT=worktree-shell`），不是 iOS Runner。可忽略或向上滚动；更新 Zed / 重装任务（`ios-runner install-zed-tasks`）后可能减轻。若只有 Zed Preview 出现，可向 Zed 反馈。

**隐私** → iOS-Runner 使用本机 Apple 工具链，不上传工程数据。见 [Security and Privacy](docs/SECURITY_AND_PRIVACY.md)。

**卸载** → `~/.ios-runner/bin/ios-runner uninstall`，Zed Extensions 里禁用插件。（CLI 不在 PATH 时用完整路径；重装 `./install-dev.sh` 后会写入 PATH）

更多：[docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md)

---

## License

MIT
