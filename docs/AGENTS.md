# Agent 速览 — iOS-Runner

## 仓库结构

```
ios-runner/
├── extension.toml      # Zed 扩展清单
├── src/lib.rs          # WASM：bootstrap CLI、install-zed-tasks
├── bin/                # 发布用 macOS CLI（aarch64 + x86_64）
├── crates/
│   ├── core/           # detect, xcodebuild, config, tasks, global_store
│   └── cli/            # ios-runner 可执行入口、mcp
└── docs/
```

## 关键约定

- **配置默认**：`~/.config/ios-runner/config.toml`（按 canonical 工程根路径索引）
- **兼容**：可读工程内 `.ios-runner.toml` / `.xcode-pilot.toml`（legacy）
- **DerivedData**：`~/.ios-runner/DerivedData/<project-hash>/`
- **Zed 任务**：全局 `~/.config/zed/tasks.json`（`ios-runner install-zed-tasks`）
- **Zed 变量**：任务脚本只用 `$HOME`、`${ZED_WORKTREE_ROOT}`，**禁止**自定义 `$变量`（会被 Zed 展开破坏）
- **扩展**：WASM 不写工作区；`bin/` 复制到 `~/.ios-runner/bin/`

## CLI 命令

| 命令 | 说明 |
|------|------|
| `ensure` | 检测工程并写入/更新全局配置 |
| `configure [--run\|--no-run]` | 交互选 scheme + destination |
| `build` / `run` | 编译 / 编译并运行 |
| `doctor` | 环境 + 工程 + CLI/任务检查 |
| `install-zed-tasks` | 全局任务 + 快捷键 |
| `install-self` | 复制当前二进制到 `~/.ios-runner/bin` |
| `uninstall` | 移除 CLI、任务、配置（可选保留） |
| `list schemes\|simulators` | JSON 列表 |
| `mcp` | MCP stdio 服务 |

## 常见修改点

| 需求 | 文件 |
|------|------|
| 工程检测 | `crates/core/src/detect.rs` |
| destination | `crates/core/src/destination.rs` |
| 交互配置 | `crates/core/src/configure.rs`, `prompt.rs` |
| build/run | `crates/core/src/xcodebuild.rs` |
| Zed 任务 | `crates/core/src/tasks.rs`, `global_tasks.rs` |
| 扩展 bootstrap | `src/lib.rs`, `crates/core/src/bootstrap.rs` |
| 新子命令 | `crates/cli/src/main.rs` |

## 构建

```bash
cd crates && cargo build --workspace
cargo install --path cli --locked
# WASM（仓库根）
cargo build --target wasm32-wasip2 --release
```

## 用户问题排查

1. `ios-runner doctor`
2. `ios-runner install-zed-tasks`（任务空 / 旧 curl 脚本）
3. 删除工程内过时 `.zed/tasks.json`
4. destination 无效 → `ios-runner configure --run`
5. CocoaPods：先 `pod install`，打开含 `.xcworkspace` 的目录
6. 真机：解锁、信任、Developer Mode、Xcode 签名 Team

## 文档入口

- 用户：[README.zh-CN.md](../README.zh-CN.md)、[QUICKSTART.md](QUICKSTART.md)
- 优化评估：[OPTIMIZATION_PROPOSALS_REVIEW.md](OPTIMIZATION_PROPOSALS_REVIEW.md)
