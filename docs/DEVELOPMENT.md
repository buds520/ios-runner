# iOS-Runner — 开发文档

## 产品定位

**iOS-Runner** 是为 [Zed](https://zed.dev/) 准备的 iOS 工程辅助工具，聚焦 **Xcode 工程的编译与运行**（不含调试、单测 gutter、SwiftUI Preview）。

- **扩展 ID**：`ios-runner`
- **CLI**：`ios-runner`
- **仓库**：[buds520/ios-runner](https://github.com/buds520/ios-runner)

## 设计原则

1. **复用 Xcode 工具链**：`xcodebuild`、`xcrun simctl` / `devicectl`。
2. **CLI 为主、扩展为辅**：WASM 扩展负责 bootstrap CLI + 全局任务；编译运行由 CLI 执行。
3. **少污染工程**：默认配置在 `~/.config/ios-runner/config.toml`；全局 Zed 任务在 `~/.config/zed/tasks.json`。
4. **CocoaPods / SPM**：通过 `xcodebuild` 统一处理；有 `Podfile` 时必须用 `.xcworkspace`。

## 架构

```
┌─────────────────────────────────────────────────────────┐
│  Zed Editor                                              │
│  ├─ Extension (WASM): bootstrap CLI → ~/.ios-runner/bin  │
│  └─ Tasks / 快捷键 → ios-runner ensure|build|run       │
└───────────────────────────┬─────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────┐
│  ios-runner CLI (crates/cli + crates/core)               │
│  ensure / configure / build / run / doctor / uninstall   │
└───────────────────────────┬─────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────┐
│  Apple: xcodebuild, simctl, devicectl                      │
└─────────────────────────────────────────────────────────┘
```

## 配置与任务

| 路径 | 说明 |
|------|------|
| `~/.config/ios-runner/config.toml` | 按工程根路径存 scheme、destination 等 |
| `~/.ios-runner/bin/ios-runner` | 扩展或 `install-self` 安装的 CLI |
| `~/.config/zed/tasks.json` | `install-zed-tasks` 写入的全局任务 |
| `~/.config/zed/keymap.json` | Cmd+Shift+E/I/R/B |
| `.ios-runner.toml` | 仅当 `IOS_RUNNER_LOCAL_CONFIG=1` 时写入工程 |

任务定义单一来源：`crates/core/src/tasks.rs` → `TASK_DEFS`。

## 本地开发

```bash
cd crates && cargo build --workspace
cargo install --path cli --locked
ios-runner install-zed-tasks

# 在 iOS 工程目录
ios-runner doctor
ios-runner ensure
ios-runner configure --run
```

### Zed Dev Extension

见 [ZED_DEV_EXTENSION.md](ZED_DEV_EXTENSION.md)。选择**仓库根目录**（含 `extension.toml`），不要选 `XcodePilotDemo`。

### 发版

```bash
./scripts/release.sh 0.2.3
```

会 bump 版本、bundle `bin/`、打 tag、推 GitHub Release、更新 extensions PR。

## 测试与 CI

```bash
cd crates && cargo test --workspace
cargo clippy --workspace -- -D warnings
```

PR / push 触发 `.github/workflows/ci.yml`。

## 版本规划

| 版本 | 内容 |
|------|------|
| v0.2.x | 全局配置、内置 CLI、destination 修复、uninstall |
| v0.3 | 构建诊断、destination switch、更多测试 |
| 未来 | Zed 动态 tasks API 落地后减少手动 `install-zed-tasks` |

## 参考

- [SWEETPAD_REFERENCE.md](SWEETPAD_REFERENCE.md)
- [PUBLISHING.md](PUBLISHING.md)
- [ZED_UX.md](ZED_UX.md)
