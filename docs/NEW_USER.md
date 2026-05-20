# 新用户指南

面向：**已有 iOS 工程**，在 Zed 里安装 iOS Runner 并使用。  
**不需要** clone 本 GitHub 仓库。

---

## 使用步骤

| 步骤 | 操作 |
|------|------|
| 1 | Zed → **Cmd+Shift+P** → **`extensions`** → 安装 **iOS Runner** |
| 2 | **File → Open Folder** → 你的工程目录（含 `.xcodeproj` / `.xcworkspace`） |
| 3 | **Cmd+Shift+R** 运行，或 **Opt+Shift+T** → **iOS-Runner: 初始化项目** |

首次运行会自动检测 Scheme、模拟器；配置保存在 `~/.config/ios-runner/config.toml`，**不会**改你的 git 工程。

---

## 工程里一开始有什么？

| 文件 | 新用户的工程 |
|------|----------------|
| `.zed/tasks.json` | **无**（跑过初始化后可能自动生成） |
| `.ios-runner.toml` | **无**（默认不用工程内配置） |

任务列表来自：扩展安装时写入的 **`~/.config/zed/tasks.json`**（全局，对所有工程生效）。

---

## 任务面板 No matches

1. 是否已 **Open Folder**（必须打开文件夹，不是单文件）  
2. 是否已安装 **iOS Runner** 扩展  
3. 仍不行 → 终端一条命令补装 CLI 与全局任务（**无需 clone**）：

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

4. 回到 Zed → Open Folder → **Cmd+Shift+R**

---

## CocoaPods

工程有 `Podfile` 时，请先在终端执行 `pod install`，并在 Zed 里 Open Folder 选择 **`.xcworkspace` 所在目录**。

---

## 维护者 / 测试

- Clone 仓库、跑 `XcodePilotDemo`、Dev Extension → [DEVELOPMENT.md](DEVELOPMENT.md)  
- 模拟清本机配置 → `./scripts/simulate-fresh-install.sh`
