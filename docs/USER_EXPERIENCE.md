# 用户体验目标

## 理想路径（目标）

1. Zed 市场安装 **iOS-Runner**
2. 打开 Xcode 工程目录
3. **自动识别** → 一键 **编译 / 运行**

## 当前 v0.1 实际路径

因 Zed 扩展尚不能写工作区、也不能动态注册任务，需要：

- 用户 **安装一次** `ios-runner` CLI
- 打开 **MCP「iOS-Runner」** 后自动 `ensure`（写 `.ios-runner.toml` + `.zed/tasks.json`）
- 用 **任务** 或快捷键 Build / Run

用户可见步骤见 **[QUICKSTART.md](QUICKSTART.md)**。

## 路线图

| 版本 | 目标 |
|------|------|
| v0.1 | 市场扩展 + MCP 自动配置 + Build/Run 任务 + `configure` 选 Scheme |
| v0.2 | 扩展内下载 CLI，无需 `cargo install` |
| v0.3 | Zed 动态 tasks → 打开文件夹即出现 Run |
| v0.4 | 扩展内 QuickPick 选 Scheme / 模拟器 |
