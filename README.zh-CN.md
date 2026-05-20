# iOS Runner

[**English**](README.md) · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 **你自己的 iOS 工程**（`xcodebuild` + 模拟器/真机）。

**环境要求：** macOS · 已安装 Xcode · 已安装 [Zed](https://zed.dev/)

---

## 快速开始

```
安装扩展 → Open Folder（你的工程）→ Cmd+Shift+R 运行
```

| 步骤 | 做什么 |
|------|--------|
| 1 | 按下面「安装方式」装好 **扩展 + CLI** |
| 2 | Zed：**File → Open Folder** → 选含 `.xcodeproj` 或 `.xcworkspace` 的目录 |
| 3 | **Cmd+Shift+R** 运行（首次可先 **Cmd+Shift+E** 初始化） |

CocoaPods 工程：先在终端 `pod install`，再 Open Folder 到 **`.xcworkspace` 所在目录**。

---

## 安装方式怎么选？

| 你的情况 | 用哪种 | 需要终端吗 |
|----------|--------|------------|
| Zed 扩展市场里能搜到 **iOS Runner** | **方式一** 扩展市场 | 一般不需要 |
| 市场还没有 / 想用最新版 | **方式二** 开发扩展 | 需要一条命令装 CLI |
| 扩展已装，但任务面板是空的 | **方式三** 只补 CLI | 一条命令 |

> **上架进度：** 审核中 → [PR #6145](https://github.com/zed-industries/extensions/pull/6145)。**目前请用方式二。**

### 扩展和 CLI 分别干什么？

| 组件 | 作用 |
|------|------|
| **Zed 扩展** | 任务面板里的「运行 / 编译 / 初始化」、快捷键 Cmd+Shift+R 等 |
| **ios-runner CLI** | 真正调用 `xcodebuild`、选 Scheme/模拟器、装到 `~/.ios-runner/bin` |

- **方式一（市场）：** 装扩展时会自动把 CLI 装进本机，**不用自己编译**。
- **方式二（开发扩展）：** 扩展不含预编译 CLI，需先执行下面的 **步骤 A**，再在 Zed 里装扩展。

---

## 方式一：Zed 扩展市场（上架后推荐）

适合：扩展市场里已能搜索安装 **iOS Runner** 的用户。

1. 打开 Zed → **Cmd+Shift+P** → 输入 `extensions` → 回车  
2. 搜索 **iOS Runner** → 点击 **Install**  
3. 等待几秒（扩展会把 CLI 装到 `~/.ios-runner/bin`）  
4. **File → Open Folder** → 你的 iOS 工程目录  
5. **Cmd+Shift+R** 运行  

**无需** clone 本仓库，**无需** 自己跑 `cargo install`。

---

## 方式二：开发扩展（当前推荐）

适合：市场未上架、或要用 GitHub 最新代码。

分两步：**先装 CLI，再在 Zed 里装扩展。**

### 步骤 A — 安装 CLI 与 Zed 全局任务（终端，只需一次）

在终端执行（**不需要** clone 仓库）：

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/install-dev.sh | bash
```

脚本会：克隆/更新源码到 `~/.ios-runner/src/ios-runner`、编译 CLI、写入 `~/.config/zed/tasks.json` 和快捷键。

已 clone 仓库时，也可在仓库根目录执行：

```bash
./install-dev.sh
```

### 步骤 B — 在 Zed 里安装开发扩展

1. Zed → **Extensions** → **Install Dev Extension**  
2. 选择目录：**仓库根目录**（里面有 `extension.toml` 和 `src/lib.rs`）  
   - 若用 `install-dev.sh`，路径通常是：`~/.ios-runner/src/ios-runner`  
   - **不要** 选 `XcodePilotDemo` 子目录  
3. **Cmd+Q** 完全退出 Zed，再重新打开  
4. **File → Open Folder** → 你的 iOS 工程  
5. **Cmd+Shift+E** 初始化，或 **Cmd+Shift+R** 直接运行  

扩展加载时若 CLI 仍未就绪，终端面板会打印 `install-dev.sh` 命令。

---

## 方式三：只补装 CLI（扩展已装、任务为空时）

适合：已安装扩展（市场或 Dev），但 **Opt+Shift+T** 里没有 iOS-Runner 任务。

1. 确认已 **Open Folder** 打开工程文件夹（不是单个 `.swift` 文件）  
2. 终端执行：

```bash
curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
```

3. **Cmd+Q** 退出 Zed → 重新打开 → 再 **Open Folder** → **Cmd+Shift+R**

---

## 安装后怎么用？

| 你想… | 操作 |
|--------|------|
| 第一次用这个工程 | **Cmd+Shift+E**（初始化，会交互选 Scheme / 模拟器或真机） |
| 编译并运行 | **Cmd+Shift+R** |
| 只编译 | **Cmd+Shift+B** |
| 换模拟器或真机 | **Cmd+Shift+I**，或终端 `ios-runner switch` |
| 看所有任务 | **Opt+Shift+T** → 搜 `iOS-Runner` |

配置保存在 **`~/.config/ios-runner/config.toml`**，按工程文件路径区分，**不会** 默认往你的 git 仓库里写配置。

工程里**可以没有** `.zed/tasks.json` — 全局任务在 `~/.config/zed/tasks.json`。

---

## 常见问题

**任务面板显示 No matches**  
→ 用 [方式三](#方式三只补装-cli扩展已装任务为空时) 补装 CLI，并确认是 **Open Folder** 而不是打开单文件。

**重复的任务（运行出现两条）**  
→ 删除工程里的 `.zed/tasks.json`，执行 `ios-runner ensure --quiet`。

**想跳过未改动的重新编译**  
→ `IOS_RUNNER_SKIP_IF_FRESH=1 ios-runner run`（可选）

**卸载**  
→ `ios-runner uninstall`，并在 Zed **Extensions** 里禁用扩展。

更多排错：[docs/NEW_USER.md](docs/NEW_USER.md) · 快捷键说明：[docs/ZED_UX.md](docs/ZED_UX.md)

---

## 维护者

`XcodePilotDemo/` 仅用于本仓库测试，不是新用户入口。  
开发 / 发版：[docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) · [docs/PUBLISHING.md](docs/PUBLISHING.md)

## License

MIT
