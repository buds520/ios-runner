# Code Review 修复说明（v0.2.x）

本文档记录针对内部 Code Review 的修复，便于后续 Agent / 新会话快速上手。

## P0（已修）

| 问题 | 修复 |
|------|------|
| `launch_artifacts` 写死 `-sdk iphonesimulator` | 按 `destination` 选择 `iphonesimulator` / `iphoneos`（`build_settings.rs`） |
| 模拟器 destination 回退硬编码 `iPhone 16` | 先 `xcodebuild -showdestinations`，再 `simctl list`，无可用模拟器则 `bail` |
| 全局 `config.toml` 并发写丢数据 | `fs2` 文件锁 + `update_global_file` 读改写（`global_store.rs`） |
| `DefaultHasher` 导致 DerivedData 目录名不稳定 | FNV-1a 64（`project_cache_id`） |

## P1（已修）

| 问题 | 修复 |
|------|------|
| `resolve_packages_quiet` 静默失败 | 失败时 `warn` 并继续编译 |
| 任务脚本 `curl` 使用 `latest` | 使用 `env!("CARGO_PKG_VERSION")` 固定 release tag（`bootstrap.rs`） |
| `lang_for_task_script` 经 `load()` 可能 bail | 只读 `load_global_file()` + project key |
| `create_config` 多次读全局文件 | 单次 `load_global_file()` 取 `defaults` |
| `xcbeautify` 默认与全局不一致 | `RunnerConfig` 字段改为 `default_true` |
| 扩展 `bootstrap_install` 每次加载都跑 | 版本标记 `~/.ios-runner/.bootstrap-v{version}`，失败打日志 |

## P2（部分）

| 问题 | 状态 |
|------|------|
| MCP 文案仍提 `.ios-runner.toml` | `ios_runner_setup` 描述已更新 |
| `cargo clippy -D warnings` | core + cli 已通过 |
| 单元测试覆盖 | 未加（仍依赖真机/Xcode 环境） |

## 环境变量（不变）

- `IOS_RUNNER_LOCAL_CONFIG=1`：同时写工程内 `.ios-runner.toml`
- `IOS_RUNNER_WRITE_PROJECT_TASKS=1`：写 `.zed/tasks.json`
- `IOS_RUNNER_LANG`：终端与任务脚本语言

## 验证

```bash
cd crates && cargo clippy --all-targets -- -D warnings
cargo build --release
```
