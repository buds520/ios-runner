# Zed 开发扩展安装排错

## 错误：`failed to compile Rust extension`

### 原因 1：未用 rustup 安装 Rust（最常见）

Zed **只支持** [rustup](https://rustup.rs/) 安装的 Rust，Homebrew 的 `rustc` 无效。

推荐：在仓库根目录运行 `./install-dev.sh`，脚本会在缺少 rustup 时**自动安装** Rust 并添加 WASM target。

手动安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version
```

### 原因 2：缺少 WASM 目标

`install-dev.sh` 会自动添加；手动执行：

```bash
rustup target add wasm32-wasip2
```

本仓库 `rust-toolchain.toml` 会提示需要该 target。

### 原因 3：选错目录

**Install Dev Extension** 必须选择 **`iOS-Runner` 仓库根目录**（含 `extension.toml` 和 `src/lib.rs`）。

正确路径示例：

```text
/Users/xj/Documents/iOS-Runner
```

### 原因 4：workspace 误编 CLI（已修复）

旧版根 `Cargo.toml` 把扩展与 CLI 放在同一 workspace，Zed 编 wasm 时会连带编译 CLI 导致失败。  
现在根目录 **仅** 扩展；CLI 在 `crates/` 下独立 workspace。

### 本地验证扩展能否编译

```bash
cd /Users/xj/Documents/iOS-Runner
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2 --release
```

成功后再在 Zed 里 Install Dev Extension。

### 编译 CLI（与扩展分开）

```bash
cd /Users/xj/Documents/iOS-Runner/crates
cargo install --path cli --locked
```

## 安装后仍无法 Run

扩展 MCP 会调用 `ios-runner mcp`；通常扩展 bootstrap 已安装 CLI 到 `~/.ios-runner/bin`。若无，见上 `cargo install` 或 GitHub Release。
