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
| 1 | 按下面两种方式之一装好 **扩展 + CLI** |
| 2 | Zed：**File → Open Folder** → 选含 `.xcodeproj` 或 `.xcworkspace` 的目录 |
| 3 | **Cmd+Shift+R** 运行（首次可先 **Cmd+Shift+U** 初始化） |

CocoaPods 工程：先在终端 `pod install`，再 Open Folder 到 **`.xcworkspace` 所在目录**。

---

## 安装方式

| 方式 | 适合谁 |
|------|--------|
| **方式一：Zed 扩展市场** | 市场里能搜到 **iOS Runner** |
| **方式二：本地扩展** | 市场未上架，或想用最新代码 |

| 组件 | 作用 |
|------|------|
| **Zed 扩展** | 任务面板里的「运行 / 编译 / 初始化」、快捷键 Cmd+Shift+R 等 |
| **ios-runner CLI** | 真正调用 `xcodebuild`、选 Scheme/模拟器，装到 `~/.ios-runner/bin` |

---

## 方式一：Zed 扩展市场

1. 打开 Zed → **Cmd+Shift+P** → 输入 `extensions` → 回车
2. 搜索 **iOS Runner** → 点击 **Install**
3. 等待几秒（扩展会自动把 CLI 装到 `~/.ios-runner/bin`）
4. **File → Open Folder** → 你的 iOS 工程目录
5. **Cmd+Shift+R** 运行

无需 clone 仓库，无需自己编译。

---

## 方式二：本地扩展

> **两个目录，别搞混：**
> | 目录 | 是什么 | 放哪 |
> |------|--------|------|
> | **插件源码** | iOS Runner 仓库（含 `extension.toml`） | 任意位置即可，如 `~/ios-runner` |
> | **你的 iOS 工程** | 你要编译运行的 App | Open Folder 打开这个，**不要**把插件 clone 进 App 里 |

> **Rust：** 方式一不需要 Rust。方式二的 `install-dev.sh` 会在缺少 rustup 时**自动安装**，并添加 Zed 所需的 `wasm32-wasip2`。

在终端执行（clone 位置随意，**不要**放在你的 App 工程目录里）：

```bash
git clone https://github.com/buds520/ios-runner.git ~/ios-runner && cd ~/ios-runner && ./install-dev.sh
```

脚本会编译 CLI 到 `~/.ios-runner/bin`，并写入 `~/.config/zed/tasks.json` 和快捷键。

然后在 Zed 里：

1. **Extensions** → **Install Dev Extension** → 选 **`~/ios-runner`**（插件源码，含 `extension.toml`）
2. **Cmd+Q** 完全退出 Zed，再重新打开
3. **File → Open Folder** → 选**你的 iOS App 工程**（含 `.xcodeproj` / `.xcworkspace`）
4. **Cmd+Shift+U** 初始化，或 **Cmd+Shift+R** 直接运行

---

## 安装后怎么用？

### 第一次打开某个 iOS 工程

1. **File → Open Folder** → 选工程根目录（含 `.xcodeproj` 或 `.xcworkspace`，不要只打开单个 `.swift` 文件）
2. **Cmd+Shift+U** — 初始化工程  
   终端里会引导你选 **Scheme**、**模拟器或真机**，配置写入 `~/.config/ios-runner/config.toml`（不进 git）
3. **Cmd+Shift+R** — 编译并运行到所选设备

> 也可以跳过第 2 步直接 **Cmd+Shift+R**，首次运行时会自动检测工程；若需手动选设备，用 **Cmd+Shift+I**。

### 日常开发

| 你想… | 操作 |
|--------|------|
| 编译并运行 | **Cmd+Shift+R** |
| 只编译 | **Cmd+Shift+B** |
| 换 Scheme / 模拟器 / 真机 | **Cmd+Shift+I** |
| 重新初始化（检测工程、补全配置） | **Cmd+Shift+U** |
| 查看全部任务 | **Opt+Shift+T** → 搜 `iOS-Runner` |
| 终端切换设备 | `ios-runner switch` |

### 快捷键一览

| 快捷键 | 动作 |
|--------|------|
| **Cmd+Shift+R** | 运行 |
| **Cmd+Shift+B** | 编译 |
| **Cmd+Shift+I** | 选择 Scheme 与设备 |
| **Cmd+Shift+U** | 初始化工程 |

> **Cmd+Shift+U** 用于初始化，避免与 Zed 侧栏等 **Cmd+Shift+E** 冲突。升级后若快捷键未变，执行 `ios-runner install-zed-tasks` 并 **Cmd+Q** 重启 Zed。

### CocoaPods 工程

终端先 `pod install`，再在 Zed 里 Open Folder 到 **`.xcworkspace` 所在目录**，然后按上面步骤操作。

---

## 常见问题

**任务面板显示 No matches**  
→ 确认是 **Open Folder** 打开工程目录（不是单个文件），然后重新执行 `./install-dev.sh` 或 `ios-runner install-zed-tasks`。

**重复的任务（运行出现两条）**  
→ 删除工程里的 `.zed/tasks.json`，执行 `ios-runner ensure --quiet`。

**想跳过未改动的重新编译**  
→ `IOS_RUNNER_SKIP_IF_FRESH=1 ios-runner run`（可选）

**卸载**  
→ `ios-runner uninstall`，并在 Zed **Extensions** 里禁用扩展。

更多排错：[docs/ZED_DEV_EXTENSION.md](docs/ZED_DEV_EXTENSION.md) · 快捷键说明：[docs/ZED_UX.md](docs/ZED_UX.md)

---

## License

MIT
