# Agent 速览 — iOS-Runner

## 仓库结构

```
ios-runner/                  # GitHub: buds520/ios-runner
├── extension.toml           # Zed 扩展（id: ios-runner）
├── src/lib.rs               # Zed WASM 扩展入口
├── crates/
│   ├── core/                # 工程检测、xcodebuild、configure、tasks
│   └── cli/                 # 用户命令 ios-runner
└── docs/
    ├── DEVELOPMENT.md
    ├── SWEETPAD_REFERENCE.md
    └── AGENTS.md            # 本文件
```

## 关键约定

- **配置**：`.ios-runner.toml`（兼容读取 `.xcode-pilot.toml`）
- **任务**：`.zed/tasks.json` 由 `init` / `configure` 生成
- **DerivedData**：`.ios-runner/DerivedData`
- **CocoaPods**：有 `Podfile` → workspace + `pod install`
- **扩展限制**：WASM 不能写工作区；配置与 tasks 由 CLI 写入

## CLI 命令

| 命令 | 说明 |
|------|------|
| `ios-runner configure` | 终端交互选择 scheme + 模拟器，写配置与 tasks |
| `ios-runner init --pick` | 同上（首次配置推荐） |
| `ios-runner init` / `ensure` | 自动检测 scheme/destination（MCP 也会触发 ensure） |
| `ios-runner build` / `run` | 编译 / 编译并启动模拟器 |
| `ios-runner list schemes` | JSON 列出 scheme |
| `ios-runner list simulators` | JSON 列出可用模拟器 |
| `ios-runner doctor` | 环境检查 |

## 常见修改点

| 需求 | 文件 |
|------|------|
| 工程检测 | `crates/core/src/detect.rs` |
| 交互配置 | `crates/core/src/configure.rs`, `prompt.rs` |
| build/run | `crates/core/src/xcodebuild.rs` |
| 模拟器列表 | `crates/core/src/simulator.rs` |
| Zed tasks | `crates/core/src/tasks.rs` |
| 新子命令 | `crates/cli/src/main.rs` |

## 构建

```bash
cd crates && cargo build --workspace
cargo install --path cli --locked
```

扩展 WASM：仓库根 `cargo build --target wasm32-wasip2 --release`

## 上架 Zed

用户 fork [zed-industries/extensions](https://github.com/zed-industries/extensions)，在 `extensions/ios-runner` 加 submodule 指向 `buds520/ios-runner`，见 `docs/PUBLISHING.md`。

## 用户问题排查

1. `ios-runner doctor`
2. scheme/模拟器不对 → `ios-runner configure`（需在终端，Zed task 会开新终端）
3. CocoaPods：`Pods/` 是否存在
4. PATH 含 `~/.cargo/bin` 的 `ios-runner`
