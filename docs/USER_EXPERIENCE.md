# 用户体验目标

## 理想路径

1. Zed 市场安装 **iOS-Runner** 扩展  
2. 重载扩展 → CLI 自动装到 `~/.ios-runner/bin`  
3. 执行一次 `ios-runner install-zed-tasks`  
4. **Open Folder** 打开 iOS 工程 → **Cmd+Shift+R** 运行  

## 当前实际路径（v0.2.x）

| 步骤 | 用户操作 |
|------|----------|
| 安装 | Zed 扩展 + `install-zed-tasks`（每台 Mac 一次） |
| 打开工程 | File → Open Folder（含 `.xcworkspace` / `.xcodeproj` 的目录） |
| 首次 | Cmd+Shift+E（Setup）或 `ios-runner ensure` |
| 选设备 | Cmd+Shift+I 或 `ios-runner configure --run` |
| 日常 | Cmd+Shift+R / **iOS-Runner: Run** |

配置保存在 `~/.config/ios-runner/config.toml`，**默认不修改 git 仓库**。

## 已知限制

- Zed 扩展 API **不能**动态注册任务 → 依赖全局 `tasks.json`  
- WASM **不能**写工作区 → 由 CLI 写配置/可选 `.zed/tasks.json`  
- 调试、SwiftUI Preview、Test gutter **不在范围**  

## 路线图

| 版本 | 目标 |
|------|------|
| v0.2.x | 内置 CLI、全局配置、快捷键、真机提示、uninstall |
| v0.3 | 构建诊断、destination 快速切换、CI 测试门禁 |
| 未来 | Zed 动态 tasks → 打开文件夹即出现 Run |

详见 [QUICKSTART.md](QUICKSTART.md)、[ZED_UX.md](ZED_UX.md)。
