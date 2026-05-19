# XcodePilotDemo

用于测试 **Xcode Pilot** 的最小 SwiftUI iOS 工程。

## 在 Zed 中打开

```bash
zed /Users/xj/Documents/iOS-Runner/XcodePilotDemo
```

或在 Zed 中：**File → Open Folder** → 选择本目录。

## 安装 Zed 扩展（方式 C）

在 Zed 中 **Install Dev Extension** 请选择父目录（含 `extension.toml` 的那一层）：

`/Users/xj/Documents/iOS-Runner`

不要选本 `XcodePilotDemo` 文件夹。若编译失败见 [docs/ZED_DEV_EXTENSION.md](../docs/ZED_DEV_EXTENSION.md)。

## 测试 Xcode Pilot

```bash
cd /Users/xj/Documents/iOS-Runner/crates && cargo install --path cli --locked
cd /Users/xj/Documents/iOS-Runner/XcodePilotDemo
xcode-pilot ensure
```

然后在 Zed：`task: spawn` → **Xcode Pilot: Run**。

若使用扩展：启用 MCP **Xcode Pilot**，首次会自动生成 `.zed/tasks.json`。