# iOS Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 **你自己的 Xcode 工程**（iOS / iPadOS / macOS，`xcodebuild` + 模拟器/真机/Mac）。

**环境要求：** macOS · Xcode · [Zed](https://zed.dev/)

---

## 快速开始

```
装扩展 → Open Folder（你的 App 工程）→ Cmd+Shift+R
```

| 快捷键 | 动作 |
|--------|------|
| **Cmd+Shift+R** | 运行 |
| **Cmd+Shift+B** | 编译 |
| **Cmd+Shift+I** | 选 Scheme / 设备（不运行） |
| **Cmd+Shift+U** | 初始化工程 |

CocoaPods：先 `pod install`，Open Folder 到 **`.xcworkspace` 所在目录**。

---

## 方式一：扩展市场

1. Zed → **Extensions** → 搜索 **iOS Runner** → Install
2. **Open Folder** → 你的 App 工程
3. **Cmd+Shift+R**

无需 clone、无需 Rust。

---

## 方式二：本地扩展

clone 放哪都行（如 `~/ios-runner`），**不要** clone 进 App 工程里。

```bash
git clone https://github.com/buds520/ios-runner.git ~/ios-runner && cd ~/ios-runner && ./install-dev.sh
```

脚本会自动装 Rust（如需要）、编译 CLI、写入 Zed 任务。

| 步骤 | 在 Zed 里做什么 |
|------|----------------|
| 1 | **Install Dev Extension** → 选 `~/ios-runner`（插件目录） |
| 2 | **Cmd+Q** 重启 → **Open Folder** → 你的 App 工程 |
| 3 | **Cmd+Shift+U** 初始化 → **Cmd+Shift+R** 运行 |

---

## 常见问题

**任务面板 No matches** → 确认 Open Folder 打开的是工程目录，再跑 `./install-dev.sh`。

**重复任务** → 删工程内 `.zed/tasks.json`，执行 `ios-runner ensure --quiet`。

**macOS 应用** → 与 iOS 相同快捷键；初始化后目标会显示「My Mac」，Cmd+Shift+R 编译并在本机启动。

**任务面板开头一堆 `HOME=…`、`ZED_*=…`** → 那是 **Zed 任务终端**在注入工程环境时打印的变量列表（`ZED_ENVIRONMENT=worktree-shell`），不是 iOS Runner。可忽略或向上滚动；更新 Zed / 重装任务（`ios-runner install-zed-tasks`）后可能减轻。若只有 Zed Preview 出现，可向 Zed 反馈。

**卸载** → `~/.ios-runner/bin/ios-runner uninstall`，Zed Extensions 里禁用插件。（CLI 不在 PATH 时用完整路径；重装 `./install-dev.sh` 后会写入 PATH）

更多：[docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md)

---

## License

MIT
