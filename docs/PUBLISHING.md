# 上架与发布（iOS-Runner）

扩展 **id**：`ios-runner`（上架后不可修改）  
市场 PR：<https://github.com/zed-industries/extensions/pull/6145>

## 一键发布（推荐）

```bash
cd /Users/xj/Documents/iOS-Runner

# 1. 本地 extensions 仓库（fork）
export EXTENSIONS_REPO="$HOME/extensions"   # 默认 ~/extensions

# 2. 发布（需 macOS）：改版本 → 编译 CLI 打入 bin/ → commit → tag → push → Release → 更新审核 PR
chmod +x scripts/*.sh
./scripts/release.sh 0.2.2
```

`release.sh` 会调用 `bundle-cli-for-extension.sh`，把 `ios-runner-aarch64-apple-darwin` / `x86_64` 放进仓库 `bin/`，随 Zed 扩展一起分发，用户**无需**首次联网下载 CLI。

仅本地打 tag、不推送：

```bash
./scripts/release.sh 0.2.0 --no-push
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
| 推送 tag `v*` | `release-cli.yml` | 构建并上传 macOS CLI 到 GitHub Release |
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

## 版本号出现在哪

| 文件 | 用途 |
|------|------|
| `extension.toml` | Zed 扩展版本 |
| `Cargo.toml` / `crates/Cargo.toml` | Rust 包版本 |
| `extensions.toml`（extensions 仓库） | 市场上架版本 |
| Git tag `vX.Y.Z` | GitHub Release + submodule 指针 |

`scripts/bump-version.sh` 会同步改 ios-runner 仓库内的 manifest。

## 首次上架 extensions（已完成）

```bash
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
gh pr create --title "Add ios-runner extension"
```

合并后在 [zed.dev/extensions](https://zed.dev/extensions) 搜索 **iOS-Runner**。
