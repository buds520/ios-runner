# iOS-Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 iOS 工程 — 思路类似 [SweetPad](https://sweetpad.hyzyla.dev/)，面向 Zed 用户。

需要 **macOS** + **Xcode**。

---

## 安装

### 方式 A — Zed 扩展（推荐）

1. Zed → **Extensions** → 搜索 **iOS-Runner** → **Install**
2. **重载扩展**一次。扩展会把内置 CLI 复制到 `~/.ios-runner/bin/ios-runner`（**无需** Rust、cargo、`curl`）
3. 写入全局任务（每台 Mac 只需一次）：

```bash
ios-runner install-zed-tasks
```

### 方式 B — 仅 CLI（开发者）

```bash
cd crates && cargo install --path cli --locked
ios-runner install-zed-tasks
```

或从 [Releases](https://github.com/buds520/ios-runner/releases) 下载 macOS 二进制。

> **cargo 是什么？** Rust 的包管理/编译工具，类似 npm。普通用户装扩展即可，不必安装 Rust。

---

## 快速上手

1. **File → Open Folder** 打开含 `.xcworkspace` / `.xcodeproj` 的目录（CocoaPods 先 `pod install`）
2. 首次：**Cmd+Shift+E**（初始化）或任务 **iOS-Runner: Setup Project**
3. **Cmd+Shift+R**（运行）或任务 **iOS-Runner: Run**

选 Scheme / 模拟器 / 真机：**Cmd+Shift+I** 或 **iOS-Runner: Select Scheme & Device**。

| 快捷键 | 作用 |
|--------|------|
| Cmd+Shift+E | 检测工程并写入配置 |
| Cmd+Shift+I | 交互选择 Scheme 与设备 |
| Cmd+Shift+R | 编译并运行 |

更多说明：[docs/ZED_UX.md](docs/ZED_UX.md) · [docs/QUICKSTART.md](docs/QUICKSTART.md)

---

## 配置放在哪

| 路径 | 说明 |
|------|------|
| `~/.config/ios-runner/config.toml` | Scheme、destination、全局默认（**默认不写进工程**） |
| `~/.ios-runner/bin/ios-runner` | 扩展或 `install-self` 安装的 CLI |
| `~/.config/zed/tasks.json` | 全局 Zed 任务（`install-zed-tasks`） |

需要工程内配置时：`export IOS_RUNNER_LOCAL_CONFIG=1` 会额外写入 `.ios-runner.toml`。

---

## 常见问题

**Run 面板是空的（No matches）**  
执行一次 `ios-runner install-zed-tasks`。新工程默认没有 `.zed/tasks.json`，不是扩展坏了。

**仍提示「正在下载 CLI」或任务脚本很旧**  
重载扩展 → `ios-runner install-zed-tasks` → 删除工程里旧的 `.zed/tasks.json`（若有）。

**destination 无效 / xcodebuild 退出码 64**  
执行 `ios-runner configure --run`，重新选择模拟器或真机。

**真机跑不起来**  
先解锁 iPhone、信任电脑、开启开发者模式；失败时会尽量给出中文提示。

**终端文案语言**  
在 `config.toml` 的 `[defaults]` 设 `language = "en"`，或 `export IOS_RUNNER_LANG=en`。

**卸载**

```bash
ios-runner uninstall                      # CLI、Zed 任务/快捷键、全局配置
ios-runner uninstall --keep-config        # 保留 ~/.config/ios-runner/
ios-runner uninstall --purge-derived-data # 同时删编译缓存
```

Zed 里的 **iOS-Runner 扩展**需在 **Extensions** 面板手动禁用/卸载。

---

## 示例工程

最小 SwiftUI 演示：[XcodePilotDemo/](XcodePilotDemo/)

---

## 文档

| 文档 | 内容 |
|------|------|
| [docs/QUICKSTART.md](docs/QUICKSTART.md) | 3 分钟上手 |
| [docs/ZED_UX.md](docs/ZED_UX.md) | 任务、快捷键、国际化 |
| [docs/PUBLISHING.md](docs/PUBLISHING.md) | 发版与市场 |
| [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) | 本地开发 |
| [docs/OPTIMIZATION_PROPOSALS_REVIEW.md](docs/OPTIMIZATION_PROPOSALS_REVIEW.md) | 优化项评估 |

---

## 许可

MIT
