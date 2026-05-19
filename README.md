# Xcode Pilot

在 [Zed](https://zed.dev/) 里开发 **iOS Xcode 工程**：扩展市场安装 → 打开工程 → **自动识别** → **编译 / 运行**（模拟器）。

## 你要的体验（产品目标）

1. Zed 扩展市场 **在线安装** Xcode Pilot  
2. 打开含 `.xcodeproj` / `.xcworkspace` 的文件夹（**支持 CocoaPods**）  
3. 扩展 **自动检测工程** 并写好运行配置  
4. 用任务或快捷键 **Run**

详细说明与路线图：[docs/USER_EXPERIENCE.md](docs/USER_EXPERIENCE.md)

## 快速开始

### 1. 安装扩展

Zed → **Extensions** → 搜索 **Xcode Pilot** → Install  

（开发阶段：Install Dev Extension → 选择本仓库根目录。）

### 2. 安装运行器（当前 v0.1 需要）

扩展通过 MCP 调用本机 `xcode-pilot`（下一版会由扩展自动下载）：

```bash
cd crates && cargo install --path cli --locked
```

### 3. 打开 iOS 工程

打开 **工程根目录**（有 `Podfile` 或 `.xcodeproj` 的那一层）。

### 4. 启用 MCP（自动检测 + 生成任务）

Settings → **MCP** → 启用 **Xcode Pilot**（扩展自带）。

首次连接时会：

- 检测 Xcode / CocoaPods workspace  
- 生成 `.xcode-pilot.toml`  
- 生成 `.zed/tasks.json`（含 **Xcode Pilot: Build / Run**）

终端里会看到 `[xcode-pilot] Xcode Pilot ready: ...` 一类提示。

### 5. 运行

- 命令面板：`task: spawn` → **Xcode Pilot: Run**  
- 或绑定快捷键（`keymap.json`）：

```json
"cmd-r": ["task::Spawn", { "task_name": "Xcode Pilot: Run" }],
"cmd-b": ["task::Spawn", { "task_name": "Xcode Pilot: Build" }]
```

CocoaPods 首次请先 **Xcode Pilot: Pod Install** 或 `pod install`。

## 能力范围

| 支持 | 不支持 (v1) |
|------|-------------|
| Xcode 工程、CocoaPods workspace | 纯 Swift Package（无 .xcodeproj） |
| 模拟器 Build + Run | 真机、调试、测试 UI |
| SPM 依赖解析 | — |

## 设计参考

构建流程参考 [SweetPad](https://github.com/sweetpad-dev/sweetpad)（VS Code）。对照表：[docs/SWEETPAD_REFERENCE.md](docs/SWEETPAD_REFERENCE.md)

```bash
brew install xcbeautify   # 可选，更易读的构建日志
```

## 开发文档

- [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) — 架构与上架  
- [docs/AGENTS.md](docs/AGENTS.md) — Agent 速览  

## 许可

MIT
