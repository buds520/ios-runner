# iOS Runner

**[English](README.md)** · **简体中文**

在 [Zed](https://zed.dev/) 里编译、运行 **你自己的 iOS 工程**（`xcodebuild` + 模拟器/真机）。

**环境要求：** macOS · 已安装 Xcode · 已安装 [Zed](https://zed.dev/)

---

## 快速开始

```
安装扩展 → Open Folder（你的工程）→ Cmd+Shift+R 运行
```


| 步骤  | 做什么                                                               |
| --- | ----------------------------------------------------------------- |
| 1   | 按下面两种方式之一装好 **扩展 + CLI**                                          |
| 2   | Zed：**File → Open Folder** → 选含 `.xcodeproj` 或 `.xcworkspace` 的目录 |
| 3   | **Cmd+Shift+R** 运行（首次可先 **Cmd+Shift+E** 初始化）                      |


CocoaPods 工程：先在终端 `pod install`，再 Open Folder 到 `**.xcworkspace` 所在目录**。

---

## 安装方式


| 方式               | 适合谁                   |
| ---------------- | --------------------- |
| **方式一：Zed 扩展市场** | 市场里能搜到 **iOS Runner** |
| **方式二：本地扩展**     | 市场未上架，或想用最新代码         |



| 组件                 | 作用                                                    |
| ------------------ | ----------------------------------------------------- |
| **Zed 扩展**         | 任务面板里的「运行 / 编译 / 初始化」、快捷键 Cmd+Shift+R 等               |
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

终端执行（需要已安装 [Rust](https://rustup.rs/)）：

```bash
git clone https://github.com/buds520/ios-runner.git && cd ios-runner && ./install-dev.sh
```

脚本会编译 CLI、写入 `~/.config/zed/tasks.json` 和快捷键。

然后在 Zed 里：

1. **Extensions** → **Install Dev Extension**
2. 选择刚才 clone 的目录（含 `extension.toml`）
3. **Cmd+Q** 完全退出 Zed，再重新打开
4. **File → Open Folder** → 你的 iOS 工程
5. **Cmd+Shift+E** 初始化，或 **Cmd+Shift+R** 直接运行

---

## 安装后怎么用？


| 你想…      | 操作                                        |
| -------- | ----------------------------------------- |
| 第一次用这个工程 | **Cmd+Shift+E**（初始化，会交互选 Scheme / 模拟器或真机） |
| 编译并运行    | **Cmd+Shift+R**                           |
| 只编译      | **Cmd+Shift+B**                           |
| 换模拟器或真机  | **Cmd+Shift+I**，或终端 `ios-runner switch`   |
| 看所有任务    | **Opt+Shift+T** → 搜 `iOS-Runner`          |


配置保存在 `**~/.config/ios-runner/config.toml`**，按工程文件路径区分，**不会** 默认往你的 git 仓库里写配置。

---

## 常见问题

**任务面板显示 No matches**  
→ 确认是 **Open Folder** 打开工程目录（不是单个文件），然后重新执行 `./install-dev.sh`。

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