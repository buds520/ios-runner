# 用户体验目标

## 新用户工程里有什么

- **没有** `.zed/tasks.json`（第一次 `ensure` 之后才可选生成）
- **没有** `.ios-runner.toml`（配置在 `~/.config/ios-runner/`）
- **有**（装扩展后）`~/.config/zed/tasks.json` — 全局任务，对所有工程生效

## 理想路径

1. 安装 **iOS Runner** 扩展 → bootstrap 写全局 tasks + CLI  
2. **Open Folder** → 任意 iOS 工程  
3. **Cmd+Shift+R** / 初始化 → `ensure` 写全局配置，可选写 `.zed/tasks.json`  

## 模拟新用户

`simulate-fresh-install.sh` 只清用户态，不往仓库塞 `.zed/tasks.json`。  
见 [NEW_USER.md](NEW_USER.md)。

## 限制

- 扩展 API 不能注册命令面板自定义项  
- 扩展只在**加载时** bootstrap；清用户态后需 Cmd+Q 重开或重装扩展  
- Zed 不能动态注册任务 → 依赖全局 `tasks.json` + 首次 ensure 写工程 tasks  
