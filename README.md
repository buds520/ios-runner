# iOS-Runner

在 [Zed](https://zed.dev/) 里 **编译 / 运行** iOS Xcode 工程（模拟器）。

---

## 用户怎么用（最短版）

**环境**：macOS + 已安装 Xcode。

1. **安装命令行**（Mac 上只需一次）  
   ```bash
   cargo install ios-runner --git https://github.com/buds520/ios-runner --locked
   ```

2. **Zed** → **Extensions** → 搜索 **iOS-Runner** → Install  

3. **打开工程**：File → Open Folder → 选含 `.xcodeproj` / `.xcworkspace` 的目录  
   - 有 `Podfile` 时先在该目录执行：`pod install`

4. **Settings** → **MCP** → 打开 **iOS-Runner**（会自动生成配置和任务）

5. **（推荐）选 Scheme / 模拟器**  
   ```bash
   ios-runner configure
   ```

6. **运行**  
   - `Cmd+Shift+P` → `task spawn` → **iOS-Runner: Run**  
   - 或终端：`ios-runner run`  
   - 只编译：**iOS-Runner: Build** / `ios-runner build`

👉 图文步骤、快捷键、排错见 **[docs/QUICKSTART.md](docs/QUICKSTART.md)**

---

## 开发 / 上架

| 文档 | 内容 |
|------|------|
| [docs/QUICKSTART.md](docs/QUICKSTART.md) | 用户上手 |
| [docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md) | Dev Extension 调试 |
| [docs/PUBLISHING.md](docs/PUBLISHING.md) | 提交 Zed 扩展市场 |
| [docs/AGENTS.md](docs/AGENTS.md) | Agent 接续开发 |

本地开发扩展：Zed → Install Dev Extension → 选本仓库根目录。

安装 CLI 源码版：

```bash
cd crates && cargo install --path cli --locked
```

## 标识

| 项 | 值 |
|----|-----|
| 市场名 | **iOS-Runner** |
| extension id | `ios-runner` |
| 项目配置 | `.ios-runner.toml` |

## 许可

MIT
