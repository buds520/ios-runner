# iOS-Runner

**English** · [**简体中文**](README.zh-CN.md)

Build and run iOS Xcode projects in [Zed](https://zed.dev/).

---

## Install

1. Zed → **Extensions** → **iOS-Runner**
2. Once per Mac, install tasks into Zed (visible in every project):

```bash
ios-runner install-zed-tasks
```

---

## Use

1. **Open Folder** on your Xcode project (`pod install` if using CocoaPods)
2. **task spawn** → **iOS-Runner: Setup Project** (first time)
3. **iOS-Runner: Run**

---

## Empty task panel?

New projects have no `.zed/tasks.json` yet — run `ios-runner install-zed-tasks` once, or `ios-runner ensure` in the project directory.

---

## License

MIT
