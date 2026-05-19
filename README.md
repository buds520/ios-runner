# iOS-Runner

在 [Zed](https://zed.dev/) 里开发 **iOS Xcode 工程**：扩展市场搜索 **iOS-Runner** → 打开工程 → **编译 / 运行**（模拟器）。

## 快速开始

### 1. 安装扩展

Zed → **Extensions** → 搜索 **iOS-Runner** → Install  

（开发：`Install Dev Extension` → 选仓库根目录 `/Users/xj/Documents/iOS-Runner`）

### 2. 安装 CLI

```bash
cd /Users/xj/Documents/iOS-Runner/crates
cargo install --path cli --locked
```

### 3. 打开 Xcode 工程并配置

```bash
cd /path/to/YourApp
ios-runner init
```

### 4. 运行

- `task: spawn` → **iOS-Runner: Run**
- 或 `ios-runner run`

```json
"cmd-r": ["task::Spawn", { "task_name": "iOS-Runner: Run" }],
"cmd-b": ["task::Spawn", { "task_name": "iOS-Runner: Build" }]
```

## 扩展 ID

| 项 | 值 |
|----|-----|
| 市场搜索名 | **iOS-Runner** |
| extension id | `ios-runner` |
| CLI | `ios-runner` |
| 项目配置 | `.ios-runner.toml` |

## 示例工程

`XcodePilotDemo/` — 可在 Zed 中打开用于测试。

## 文档

- [docs/PUBLISHING.md](docs/PUBLISHING.md) — 上架 Zed 市场  
- [docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md) — Dev Extension 排错  

## 许可

MIT
