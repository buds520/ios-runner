# Zed 里的操作方式（设置 vs 一级命令）

## 终端提示语言

| 方式 | 示例 |
|------|------|
| 工程 `.ios-runner.toml` | `language = "en"` 或 `language = "zh-CN"` |
| 环境变量（优先） | `export IOS_RUNNER_LANG=en` |
| Zed 任务 `env` | 在 `tasks.json` 里对任务加 `"env": { "IOS_RUNNER_LANG": "en" }` |

改完后重新跑任务，或执行 `ios-runner install-zed-tasks` 刷新全局任务脚本。

**注意**：Zed 会在执行前展开任务里的 `$变量`（仅保留环境变量与 `ZED_*`）。任务脚本因此使用 `$HOME/.ios-runner/bin/ios-runner`，不要用自定义名如 `$ir_bin`（会被替换成空，出现 `chmod: : No such file or directory`）。

### 仍提示「正在下载」？

1. 刷新全局任务：`ios-runner install-zed-tasks`（或重载扩展）
2. **删除工程内** `.zed/tasks.json`（会优先于全局任务，旧文件里常有 curl 下载脚本）
3. 确认 CLI 正常：`ls -l ~/.ios-runner/bin/ios-runner`（应约 2MB，不是几十 KB）
4. 重装 CLI：在本仓库运行 `./install-dev.sh`，或执行当前 CLI 的 `ios-runner install-self`

## Marketplace 扩展行为

从市场安装扩展后，Zed 会在需要启动 MCP server 时从 GitHub Release 下载匹配的 macOS binary。扩展加载本身不写 `tasks.json`、`keymap.json`，也不向 `~/.ios-runner/bin` 复制文件。

| 步骤 | 说明 |
|------|------|
| 装扩展 | Zed 扩展市场安装 **iOS Runner** |
| 打开工程 | Agent 面板可启动 iOS-Runner MCP tools |
| 点 Run | 若要使用快捷键/Run 面板任务，先安装 CLI 并执行 `ios-runner install-zed-tasks` |

本地开发时，`install-dev.sh` 仍会安装 CLI、写入任务和快捷键，适合日常手动测试。

## 全局配置（默认，不改工程目录）

| 文件 | 作用 |
|------|------|
| **`~/.config/ios-runner/config.toml`** | 每个工程的 scheme / 设备 / Team；`[defaults]` 为全局默认 |
| **`~/.config/zed/tasks.json`** | 全局 Zed 任务（`ios-runner install-zed-tasks`） |
| **`~/.config/zed/keymap.json`** | 全局快捷键 |
| **`~/.ios-runner/DerivedData/<id>/`** | 编译缓存（不在工程里） |

`ios-runner ensure` / `configure` **默认不会**写入 `.ios-runner.toml` 或 `.zed/tasks.json`。

若仍需旧行为（写入工程目录）：

```bash
export IOS_RUNNER_LOCAL_CONFIG=1          # 同时写 .ios-runner.toml
export IOS_RUNNER_WRITE_PROJECT_TASKS=1 # 同时写 .zed/tasks.json
```

已有 `.ios-runner.toml` 会在首次 `ensure` 时**迁移**到全局配置。

模板见 [templates/global-config.toml.example](../templates/global-config.toml.example)。

## Zed 设置面板

Zed **目前不允许**扩展单独做一个「iOS-Runner 设置」标签页。运行相关选项以 **`~/.config/ios-runner/config.toml`** 为准。

## 一级命令（不经过 task: spawn 搜索）

扩展 **不能**向命令面板注册「iOS-Runner: Run」这种独立条目（Zed 扩展 API 无 `contribute.commands`）。

可用替代：

### 1. 快捷键（推荐，等同一级指令）

执行一次：

```bash
ios-runner install-zed-tasks
```

会写入 `~/.config/zed/keymap.json`：

| 快捷键 | 动作 |
|--------|------|
| **Cmd+Shift+R** | 运行 |
| **Cmd+Shift+B** | 编译 |
| **Cmd+Shift+I** | 选择 Scheme 与运行目标 |
| **Cmd+Shift+U** | 初始化项目（ensure） |

### 2. Run 面板

安装全局任务后，**Run** 标签里直接点 **iOS-Runner: Run**，无需 `task: spawn` 搜索。

### 3. 重复上一次

`task: rerun`（默认快捷键因 keymap 而异）— 跑过一次 Run 后可一键重跑。

### 4. Agent / MCP

在 Agent 里可用 MCP 工具（`ios_runner_run` 等）；面向 AI，不是日常 Run 按钮。

## 路线图

| 版本 | 目标 |
|------|------|
| 当前 | 全局任务 + 全局快捷键 + `.ios-runner.toml` |
| 待 Zed API | 扩展动态注册任务、设置项进 Settings Editor |
| 远期 | 扩展内 QuickPick 选 Scheme/设备（需 Zed 支持） |
