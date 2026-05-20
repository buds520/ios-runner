# iOS Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 **你自己的 iOS 工程**。需要 **macOS** + **Xcode**。

---

## 新用户（3 步）

```
装扩展 → Open Folder（你的工程）→ Cmd+Shift+R
```

| 步骤 | 操作 |
|------|------|
| 1 | 打开 **Zed** → **Cmd+Shift+P** → 输入 **`extensions`** → 安装 **iOS Runner** |
| 2 | **File → Open Folder** → 选择你的 iOS 工程目录（含 `.xcodeproj` 或 `.xcworkspace` 的那一层） |
| 3 | **Cmd+Shift+R** 运行；首次可先 **Opt+Shift+T** → **iOS-Runner: 初始化项目** |

CocoaPods 工程：请先在系统终端执行一次 `pod install`（Xcode 常规步骤）。

扩展安装时会自动配置 CLI 和 Zed 全局任务。你的工程里**不会**预先带有 `.zed/tasks.json`，这很正常。

---

## 任务面板是空的？

先确认已 **Open Folder** 打开工程（不是只打开单个文件）。

若仍无任务，在终端执行**一条命令**（无需 clone 本仓库）：

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

然后回到 Zed：**Open Folder** → 你的工程 → **Cmd+Shift+R**。

---

## 快捷键

| 键 | 动作 |
|----|------|
| Cmd+Shift+R | 运行 |
| Cmd+Shift+E | 初始化项目 |
| Cmd+Shift+B | 编译 |
| Cmd+Shift+I | 选 Scheme / 设备 |

---

## 开发与测试本仓库

维护者 clone 本仓库、跑 Demo 或改扩展代码，见 [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)。  
Demo 工程 `XcodePilotDemo/` **仅用于测试**，不是新用户入口。

---

## 链接

- [新用户说明 / 排错](docs/NEW_USER.md)
- [GitHub](https://github.com/buds520/ios-runner)
