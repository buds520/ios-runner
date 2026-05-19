# 上架 Zed 扩展市场

官方流程：[Developing Extensions — Publishing](https://zed.dev/docs/extensions/developing-extensions#publishing-your-extension)

## 前置条件

- [x] 根目录 `extension.toml`、`LICENSE`（MIT）
- [x] 扩展可编译：`cargo build --target wasm32-wasip2 --release`
- [ ] **公开 GitHub 仓库**（建议个人账号，非 Organization）
- [ ] 向 [zed-industries/extensions](https://github.com/zed-industries/extensions) 提 PR

## 第一步：创建 GitHub 仓库并推送

```bash
cd /Users/xj/Documents/iOS-Runner

# 1. 登录 GitHub CLI（只需一次）
gh auth login

# 2. 把 extension.toml 里的 repository 改成你的地址，例如：
#    repository = "https://github.com/你的用户名/xcode-pilot"

# 3. 创建远程仓库并推送
gh repo create xcode-pilot --public --source=. --remote=origin --push
```

若已手动建仓库：

```bash
git remote add origin https://github.com/你的用户名/xcode-pilot.git
git push -u origin main
```

## 第二步：Fork 并克隆 zed-industries/extensions

```bash
cd ~/Developer   # 任意目录
gh repo fork zed-industries/extensions --clone
cd extensions
git submodule init
git submodule update
```

## 第三步：添加本扩展为 submodule

```bash
git submodule add https://github.com/你的用户名/xcode-pilot.git extensions/xcode-pilot
git add extensions/xcode-pilot .gitmodules
```

在仓库根目录 `extensions.toml` **顶部**增加（版本与 `extension.toml` 一致）：

```toml
[xcode-pilot]
submodule = "extensions/xcode-pilot"
version = "0.1.0"
```

## 第四步：排序并提交 PR

```bash
pnpm install   # 若仓库要求
pnpm sort-extensions
git add extensions.toml .gitmodules extensions/xcode-pilot
git commit -m "Add xcode-pilot extension"
git push origin HEAD
gh pr create --title "Add xcode-pilot extension" --body "$(cat <<'EOF'
## Summary
- Xcode Pilot: build and run iOS Xcode projects in Zed via xcodebuild
- Supports CocoaPods workspaces and SPM resolve in Xcode projects
- Bundled MCP context server; project tasks via xcode-pilot CLI

## Test plan
- [ ] Install extension from PR build / dev
- [ ] Open sample Xcode project, run Build/Run tasks
- [ ] macOS + Xcode required

EOF
)"
```

## 合并后

Zed 团队合并 PR 后，扩展会出现在 [zed.dev/extensions](https://zed.dev/extensions) 搜索 **Xcode Pilot**。

## 审核注意

1. `repository` 必须是真实可访问的 HTTPS 地址  
2. 扩展 **id** `xcode-pilot` 上架后不可改  
3. 不要在本扩展仓库内捆绑 language server 二进制  
4. 说明文档写清：需 macOS、Xcode、以及 `xcode-pilot` CLI（或后续版本自动下载）
