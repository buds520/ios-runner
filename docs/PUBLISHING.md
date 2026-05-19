# 上架 Zed 扩展市场（iOS-Runner）

扩展 **id**：`ios-runner`（上架后不可修改）  
市场显示名：**iOS-Runner**（便于用户搜索）

## 推送 GitHub

```bash
cd /Users/xj/Documents/iOS-Runner
# 修改 extension.toml 中 repository 为你的地址
gh auth login
gh repo create ios-runner --public --source=. --remote=origin --push
# 仓库地址: https://github.com/buds520/ios-runner
```

## 向 zed-industries/extensions 提 PR

```bash
git submodule add https://github.com/buds520/ios-runner.git extensions/ios-runner
```

`extensions.toml` 增加：

```toml
[ios-runner]
submodule = "extensions/ios-runner"
version = "0.1.0"
```

```bash
pnpm sort-extensions
gh pr create --title "Add ios-runner extension"
```

合并后在 [zed.dev/extensions](https://zed.dev/extensions) 搜索 **iOS-Runner**。
