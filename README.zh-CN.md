# iOS Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 **你自己的 iOS 工程**（`xcodebuild` + 模拟器/真机）。

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
| **Cmd+Shift+I** | 选 Scheme / 设备 |
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

**macOS 工程** → iOS Runner 仅支持 iOS / iPadOS，Mac 应用请在 Xcode 中编译。

**卸载** → `~/.ios-runner/bin/ios-runner uninstall`，Zed Extensions 里禁用插件。（CLI 不在 PATH 时用完整路径；重装 `./install-dev.sh` 后会写入 PATH）

更多：[docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md)

---

## License

MIT
