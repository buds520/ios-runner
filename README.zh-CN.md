# iOS-Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 iOS 工程。

---

## 安装

1. Zed → **Extensions** → 安装 **iOS-Runner**（或 Dev Extension 选本仓库）  
   扩展加载时会尝试下载 CLI 并写入全局任务；若尚未发布 GitHub Release，请用下方「开发者」方式安装 CLI。
2. **一次性**把任务装进 Zed（所有工程都会出现，只需做一次；扩展已装可跳过）：

```bash
# 若已有 ios-runner（见下「开发者」）；或先 Release 下载后执行
ios-runner install-zed-tasks
```

没有 `ios-runner` 时，可先在 Zed **终端**里用开发者方式安装 CLI，再执行上面命令。

---

## 使用

1. **File → Open Folder** 打开含 `.xcodeproj` / `.xcworkspace` 的目录（CocoaPods 先 `pod install`）
2. `Cmd+Shift+P` → **task spawn** → 选 **iOS-Runner: Setup Project**（首次）
3. 再选 **iOS-Runner: Run**

---

## 面板里是空的（No matches）？

Zed 只显示 **`.zed/tasks.json`** 或 **全局 `~/.config/zed/tasks.json`** 里的任务。  
**新工程默认没有这些文件**，所以 Run 面板是空的，不是插件坏了。

任选一种办法：

| 办法 | 操作 |
|------|------|
| **推荐** | 执行一次 `ios-runner install-zed-tasks`（见上）；重装扩展后建议再执行一次以更新任务脚本 |
| 仅当前工程 | Zed 开终端，在项目目录：`ios-runner ensure` |
| 临时一次 | 任务框里直接输入命令，按 **Opt+Enter** 运行（见 Zed 文档 oneshot task） |

---

## 开发者（需 Rust / cargo）

```bash
cd crates && cargo install --path cli --locked
ios-runner install-zed-tasks
```

---

## 许可

MIT
