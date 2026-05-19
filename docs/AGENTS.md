# Agent 速览 — Xcode Pilot

## 仓库结构

```
xcode-pilot/                 # 建议重命名 Git 仓库
├── extension.toml           # Zed 扩展清单（id: xcode-pilot）
├── src/lib.rs               # Zed WASM 扩展入口
├── crates/
│   ├── core/                # 工程检测、xcodebuild、写 tasks
│   └── cli/                 # 用户命令 xcode-pilot
└── docs/
    ├── DEVELOPMENT.md       # 设计与版本规划
    └── AGENTS.md            # 本文件
```

## 关键约定

- **配置**：项目根 `.xcode-pilot.toml`
- **任务**：`.zed/tasks.json` 由 `xcode-pilot init` 生成，不手写
- **构建产物**：`.xcode-pilot/DerivedData`（已 gitignore）
- **CocoaPods**：有 `Podfile` 时必须用 workspace，且先 `pod install`
- **扩展限制**：Zed WASM 不能写工作区文件；逻辑放在 CLI

## SweetPad 对照

实现 iOS 构建/运行时优先阅读 `docs/SWEETPAD_REFERENCE.md`，避免重复造轮子或偏离业界做法。

## 常见修改点

| 需求 | 文件 |
|------|------|
| 改检测规则 | `crates/core/src/detect.rs` |
| 改 build/run | `crates/core/src/xcodebuild.rs` |
| 改生成 tasks | `crates/core/src/tasks.rs` |
| 新子命令 | `crates/cli/src/main.rs` |

## 构建

```bash
cargo build --workspace
cargo test --workspace   # 若有测试
```

## 用户问题排查

1. `xcode-pilot doctor`
2. CocoaPods：`Pods/` 是否存在
3. `destination` 是否与已安装模拟器名称一致（改 `.xcode-pilot.toml`）
4. Zed 是否已将 `xcode-pilot` 加入 PATH
