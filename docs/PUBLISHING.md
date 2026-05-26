# 上架与发布（iOS-Runner）

扩展 **id**：`ios-runner`（上架后不可修改）  
市场 PR：<https://github.com/zed-industries/extensions/pull/6145>

## 一键发布（推荐）

```bash
cd /Users/xj/Documents/iOS-Runner

# 1. 本地 extensions 仓库（fork）
export EXTENSIONS_REPO="$HOME/extensions"   # 默认 ~/extensions

# 2. 发布：改版本 → commit → tag → push → Release assets → 更新审核 PR
chmod +x scripts/*.sh
./scripts/release.sh 0.2.2
```

`release.sh` 不再把 macOS CLI 二进制提交进扩展仓库。Zed 扩展按需从 GitHub Release 下载匹配的 MCP server binary，这更贴近 Zed MCP extension 的常见发布方式，也避免 registry PR 携带平台二进制。

仅本地打 tag、不推送：

```bash
./scripts/release.sh 0.2.0 --no-push
```

发布脚本会在打 commit/tag 前执行 release-readiness 检查，确认 `extension.toml`、`Cargo.toml`、`crates/Cargo.toml` 和 `CHANGELOG.md` 的版本一致。也可以单独运行：

```bash
./scripts/check-release-readiness.sh 0.2.0
```

不更新 Zed extensions PR：

```bash
./scripts/release.sh 0.2.0 --skip-extensions
```

单独更新审核 PR（已打过 tag）：

```bash
./scripts/update-zed-extensions-pr.sh 0.2.0
```

## GitHub Actions 自动发布

| 事件 | 工作流 | 作用 |
|------|--------|------|
| 推送 tag `v*` | `release-cli.yml` | 构建并上传 macOS MCP server binary 到 GitHub Release |
| 推送 tag `v*` | `publish-zed-extension.yml` | 更新 `buds520/extensions` 的 submodule + 在 PR 留言 |

### 配置仓库 Secret（仅需一次）

在 **buds520/ios-runner** → Settings → Secrets → Actions：

| Secret | 说明 |
|--------|------|
| `EXTENSIONS_DEPLOY_TOKEN` | GitHub PAT，`repo` 权限，能 push `buds520/extensions` |

生成 PAT：GitHub → Settings → Developer settings → Fine-grained token → Repository access: `buds520/extensions` + `buds520/ios-runner` → Contents: Read and write.

配置后，以后只需：

```bash
git push origin main
./scripts/release.sh 0.2.2   # 或只 git push tag，Actions 会跟
```

也可在 Actions 里手动运行 **Publish Zed Extension**，填写版本号。

## 发布前 Smoke Test

默认 CocoaPods demo smoke 覆盖 `doctor`、`ensure`、`switch --list` 和 CocoaPods workspace 路径：

```bash
./scripts/smoke-test-demo.sh
```

如果当前机器已安装 Xcode、xcodegen、CocoaPods，并允许真实 build，再跑一次带编译的 smoke：

```bash
IOS_RUNNER_SMOKE_BUILD=1 ./scripts/smoke-test-demo.sh
```

如果 demo 提示缺少 `.xcworkspace`，先运行 **iOS-Runner: Pod Install** 或 `pod install`，生成 workspace 后重新初始化项目或运行。

## 版本号出现在哪

| 文件 | 用途 |
|------|------|
| `extension.toml` | Zed 扩展版本 |
| `Cargo.toml` / `crates/Cargo.toml` | Rust 包版本 |
| `extensions.toml`（extensions 仓库） | 市场上架版本 |
| Git tag `vX.Y.Z` | GitHub Release + submodule 指针 |

`scripts/bump-version.sh` 会同步改 ios-runner 仓库内的 manifest。

## 首次上架 extensions

```bash
git checkout -B add-ios-runner-v2 upstream/main
git submodule add https://github.com/buds520/ios-runner.git extensions/ios-runner
```

`extensions.toml`：

```toml
[ios-runner]
submodule = "extensions/ios-runner"
version = "0.2.0"
```

```bash
pnpm sort-extensions
git commit -m "Add ios-runner extension"
git push -u origin add-ios-runner-v2
gh pr create --repo zed-industries/extensions --head buds520:add-ios-runner-v2 --title "Add ios-runner extension"
```

合并后在 [zed.dev/extensions](https://zed.dev/extensions) 搜索 **iOS-Runner**。
