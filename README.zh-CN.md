# iOS Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 **你自己的 iOS 工程**。需要 **macOS** + **Xcode**。

---

## 安装（二选一）

### 方式一：扩展市场（推荐）

Zed 收录后：

1. **Cmd+Shift+P** → 输入 `extensions` → 打开扩展面板 → 搜索 **iOS Runner** → **安装**
2. **File → Open Folder** → 你的 iOS 工程目录（含 `.xcodeproj` 或 `.xcworkspace` 的那一层）

**上架状态：** 审核中 — [PR #6145](https://github.com/zed-industries/extensions/pull/6145)。合并前请用 **方式二**，或按下方「任务为空」安装 CLI。

### 方式二：开发扩展（未上架 / 要用最新源码）

```bash
git clone https://github.com/buds520/ios-runner.git
```

1. Zed → **Extensions** → **Install Dev Extension** → 选 **仓库根目录**（含 `extension.toml`，不要选 `XcodePilotDemo`）
2. **File → Open Folder** → 你的工程

可选：本机编译 CLI — `cd ios-runner/crates && cargo install --path cli --locked`  
或执行 `./scripts/install.sh`（见 [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)）。

---

## 使用（两种方式相同）

| 步骤 | 操作 |
|------|------|
| 打开工程 | **Open Folder** 到含 `.xcodeproj` / `.xcworkspace` 的目录（CocoaPods 请先 `pod install`） |
| 首次 | **Cmd+Shift+E** 或任务 **iOS-Runner: 初始化项目** |
| 运行 | **Cmd+Shift+R** 或任务 **iOS-Runner: 运行** |
| 选设备 | **Cmd+Shift+I** 或 **iOS-Runner: 选择 Scheme 与设备** |

工程里**可以没有** `.zed/tasks.json`，扩展会配置 `~/.config/zed/tasks.json` 全局任务。

---

## 任务面板是空的？

1. 确认已 **Open Folder** 打开工程根目录（不是单个文件）。
2. 终端执行（无需 clone 仓库）：

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

3. **Cmd+Q** 退出 Zed 后重新打开，或重装扩展。

---

## 维护者

`XcodePilotDemo/` 仅用于测试。开发说明：[docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) · 上架：[docs/PUBLISHING.md](docs/PUBLISHING.md)

## 链接

- [新用户 / 排错](docs/NEW_USER.md)
- [GitHub](https://github.com/buds520/ios-runner)
