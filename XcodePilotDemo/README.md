# XcodePilotDemo

用于测试 **iOS-Runner** 的最小 SwiftUI iOS 工程。

## 在 Zed 中打开

**File → Open Folder** → 选择本目录 `XcodePilotDemo`。

或终端：

```bash
zed /path/to/iOS-Runner/XcodePilotDemo
```

## 使用 iOS-Runner

1. 安装 [Zed 扩展](../README.zh-CN.md#安装) 并执行一次 `ios-runner install-zed-tasks`
2. 在本目录终端（可选）：

```bash
ios-runner ensure          # 或 ios-runner configure --run
ios-runner run
```

3. 或在 Zed：**Cmd+Shift+R** / 任务 **iOS-Runner: Run**

## Dev Extension（开发扩展本身时）

在 Zed 里 **Install Dev Extension** 请选择**仓库根目录**（含 `extension.toml` 的那一层）：

```
/path/to/iOS-Runner
```

不要选本 `XcodePilotDemo` 文件夹。编译失败见 [docs/ZED_DEV_EXTENSION.md](../docs/ZED_DEV_EXTENSION.md)。

## 本地编译 CLI（可选）

```bash
cd ../crates && cargo install --path cli --locked
cd ../XcodePilotDemo && ios-runner doctor
```
