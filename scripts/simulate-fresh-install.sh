#!/usr/bin/env bash
# Remove user-level iOS-Runner / Zed state. Repository stays as after `git clone`.
#
# Usage:
#   ./scripts/simulate-fresh-install.sh           # clean ~/.ios-runner, ~/.config/zed (iOS-Runner entries), etc.
#   ./scripts/simulate-fresh-install.sh --build   # also remove crates/target (optional)
#
# After this script: do NOT run any other install script. Continue entirely in Zed (see README.zh-CN.md).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

DO_BUILD=0
for arg in "$@"; do
  case "$arg" in
    --build) DO_BUILD=1 ;;
    -h|--help)
      sed -n '2,8p' "$0"
      exit 0
      ;;
    *) echo "Unknown: $arg" >&2; exit 1 ;;
  esac
done

echo "→ iOS-Runner: simulate fresh install (repo kept at $ROOT)"

if command -v ios-runner >/dev/null 2>&1; then
  ios-runner uninstall --purge-derived-data
elif [[ -x "$ROOT/crates/target/release/ios-runner" ]]; then
  "$ROOT/crates/target/release/ios-runner" uninstall --purge-derived-data
else
  echo "→ ios-runner not in PATH; removing paths manually"
  rm -f "${HOME}/.ios-runner/bin/ios-runner" 2>/dev/null || true
  rm -f "${HOME}"/.ios-runner/.bootstrap-* 2>/dev/null || true
  rm -rf "${HOME}/.ios-runner/DerivedData" "${HOME}/.config/ios-runner"
  rm -f "${HOME}/.config/zed/keymap.json" 2>/dev/null || true
  if [[ -f "${HOME}/.config/zed/tasks.json" ]]; then
    python3 - <<'PY' 2>/dev/null || rm -f "${HOME}/.config/zed/tasks.json"
import json, os
path = os.path.expanduser("~/.config/zed/tasks.json")
with open(path) as f:
    tasks = json.load(f)
tasks = [t for t in tasks if not str(t.get("label", "")).startswith("iOS-Runner:")]
if tasks:
    with open(path, "w") as f:
        json.dump(tasks, f, indent=2)
        f.write("\n")
else:
    os.remove(path)
PY
  fi
fi

rm -f "${HOME}/.cargo/bin/ios-runner" 2>/dev/null || true

rmdir "${HOME}/.ios-runner/bin" 2>/dev/null || true
rmdir "${HOME}/.ios-runner" 2>/dev/null || true

if [[ "$DO_BUILD" -eq 1 ]]; then
  echo "→ Remove local build artifacts"
  rm -rf "$ROOT/crates/target" "$ROOT/target" "$ROOT/extension.wasm"
fi

echo ""
echo "✓ User state cleared (same as a new Mac). Do not run ios-runner or bootstrap scripts."
echo ""
echo "Continue in Zed only (Zed 没有 Reload Window 命令):"
echo ""
echo "  扩展尚未安装:"
echo "    1. Cmd+Shift+P → extensions → 安装 iOS Runner"
echo "    2. Open Folder → 你的 iOS 工程"
echo "    3. Opt+Shift+T → 初始化  或  Cmd+Shift+R"
echo ""
echo "  扩展已安装（刚清过本机配置）:"
echo "    1. Cmd+Q 完全退出 Zed 再打开（或 Cmd+Shift+P → reload workspace）"
echo "    2. Open Folder → 你的 iOS 工程"
echo "    3. Opt+Shift+T → 初始化  或  Cmd+Shift+R"
echo ""
echo "  任务来自扩展写入的全局 ~/.config/zed/tasks.json（不是工程内 .zed/）。"
echo ""
echo "详见 README.zh-CN.md"
