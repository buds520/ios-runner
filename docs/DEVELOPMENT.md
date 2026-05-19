# Xcode Pilot — 开发文档

## 产品定位

**Xcode Pilot** 是为 [Zed](https://zed.dev/) 准备的 Apple 平台开发辅助扩展，聚焦 **Xcode 工程的编译与运行**（不含调试、单测 gutter、SwiftUI Preview）。

- **扩展 ID**：`xcode-pilot`
- **CLI 命令**：`xcode-pilot`
- **上架目标**：[zed-industries/extensions](https://github.com/zed-industries/extensions)

## 设计原则

1. **复用 Xcode 工具链**：`xcodebuild`、`xcrun simctl`，不引入 xcede 等第三方构建器。
2. **CLI 为主、扩展为辅**：Zed 扩展 API 暂不支持动态 tasks / 写工作区文件；由 CLI 生成 `.zed/tasks.json` 与 `.xcode-pilot.toml`。
3. **工程类型**：以 `.xcodeproj` / `.xcworkspace` 为主；CocoaPods、工程内 SPM 通过 `xcodebuild` 统一处理。

## 架构

```
┌─────────────────────────────────────────────────────────┐
│  Zed Editor                                              │
│  ├─ Extension (WASM): 发现、文档、后续 MCP/动态 tasks     │
│  └─ Tasks: 调用 `xcode-pilot build` / `xcode-pilot run` │
└───────────────────────────┬─────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────┐
│  xcode-pilot CLI                                         │
│  ├─ init    → 检测工程、写配置、生成 .zed/tasks.json      │
│  ├─ build   → xcodebuild build                           │
│  ├─ run     → build + simctl install + launch            │
│  ├─ doctor  → 检查 xcode-select / simctl / pod 等        │
│  └─ list    → schemes / destinations（JSON）             │
└───────────────────────────┬─────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────┐
│  Apple: xcodebuild, xcrun simctl                         │
└─────────────────────────────────────────────────────────┘
```

## CocoaPods / Swift Package

| 场景 | 行为 |
|------|------|
| 存在 `Podfile` | 优先使用 `*.xcworkspace`（非 `project.xcworkspace`）；`init` 提示执行 `pod install` |
| 工程内 SPM | `build`/`run` 前可执行 `-resolvePackageDependencies`；`init` 生成可选 task |
| 仅 `Package.swift`、无 Xcode 工程 | **不在 v1 范围**；文档说明请用 Swift 扩展 + `swift build` |

## 配置文件

### `.xcode-pilot.toml`（项目根）

```toml
kind = "workspace"           # "workspace" | "project"
path = "MyApp.xcworkspace"
scheme = "MyApp"
configuration = "Debug"
destination = "platform=iOS Simulator,name=iPhone 16"
derived_data = ".xcode-pilot/DerivedData"
```

### `.zed/tasks.json`（由 `xcode-pilot init` 生成）

| Task | 说明 |
|------|------|
| Xcode Pilot: Build | `xcode-pilot build` |
| Xcode Pilot: Run | `xcode-pilot run` |
| Xcode Pilot: Resolve SPM | `xcode-pilot resolve-packages` |
| Xcode Pilot: Pod Install | `pod install`（仅当存在 Podfile） |

## 本地开发

```bash
# 构建 CLI
cargo build -p xcode-pilot-cli

# 安装到 PATH（开发）
cargo install --path crates/cli --locked

# 在 Xcode 工程根目录
xcode-pilot doctor
xcode-pilot init
xcode-pilot build
xcode-pilot run
```

### Zed Dev Extension

1. 安装 Rust（rustup）
2. Zed → Extensions → Install Dev Extension → 选择仓库根目录
3. 确保 `xcode-pilot` 在 PATH（`cargo install --path crates/cli`）
4. 在工程中执行 `xcode-pilot init`，绑定 `cmd-b` / `cmd-r`（见 README）

## 上架清单

- [ ] 仓库根目录 `LICENSE`（MIT / Apache-2.0 / BSD-2/3）
- [ ] `extension.toml` 字段完整且 **id 唯一**
- [ ] 在独立仓库发布，向 `zed-industries/extensions` 提 PR（submodule + `extensions.toml`）
- [ ] 扩展声明 `process:exec`（`xcodebuild` 等）— 用户可在设置中授权

## 版本规划

| 版本 | 内容 |
|------|------|
| v0.1 | init / build / run / doctor / list；CocoaPods & SPM resolve |
| v0.2 | MCP context server（Agent 内 init/doctor） |
| v0.3 | Zed 动态 tasks API 落地后，扩展内自动发现 scheme |

## SweetPad 参考

VS Code 扩展 [sweetpad-dev/sweetpad](https://github.com/sweetpad-dev/sweetpad) 是同类产品的成熟实现。技术对照与已移植项见 [docs/SWEETPAD_REFERENCE.md](./SWEETPAD_REFERENCE.md)。

## 相关链接

- [Zed Tasks](https://zed.dev/docs/tasks)
- [Zed Developing Extensions](https://zed.dev/docs/extensions/developing-extensions)
- [xcodebuild destination](https://mokacoding.com/blog/xcodebuild-destination-options/)
