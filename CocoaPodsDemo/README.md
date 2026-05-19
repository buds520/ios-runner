# CocoaPodsDemo

用于测试 **iOS-Runner** 的 CocoaPods 工程（`Podfile` → `pod install` → `.xcworkspace`）。

## 依赖

- Xcode、CocoaPods（`gem install cocoapods` 或 `brew install cocoapods`）
- 可选：[XcodeGen](https://github.com/yonaskolb/XcodeGen)（`brew install xcodegen`）用于生成 `.xcodeproj`

## 首次准备

```bash
cd /Users/xj/Documents/iOS-Runner/CocoaPodsDemo

# 1. 生成 Xcode 工程（若无 CocoaPodsDemo.xcodeproj）
xcodegen generate

# 2. 安装 Pods（生成 CocoaPodsDemo.xcworkspace）
pod install
```

## 用 iOS-Runner 测试

```bash
ios-runner doctor
ios-runner configure    # 应识别为 workspace，scheme = CocoaPodsDemo
ios-runner run
```

Zed：打开 **CocoaPodsDemo** 文件夹 → 先跑任务 **iOS-Runner: Pod Install** → 再 **Run**。

## Pod

- [SwiftyJSON](https://github.com/SwiftyJSON/SwiftyJSON) — 点击按钮会 `print` JSON 到 Zed 终端。
