#!/usr/bin/env bash
# Install ios-runner CLI + Zed tasks on macOS (no git clone required).
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/buds520/ios-runner/main/scripts/install-cli.sh | bash
set -euo pipefail

VERSION="${IOS_RUNNER_VERSION:-0.3.0}"
REPO="buds520/ios-runner"
INSTALL_BIN="${HOME}/.ios-runner/bin/ios-runner"

if [[ "$(uname -s)" != Darwin ]]; then
  echo "iOS Runner requires macOS." >&2
  exit 1
fi

case "$(uname -m)" in
  arm64) ASSET="ios-runner-aarch64-apple-darwin" ;;
  x86_64) ASSET="ios-runner-x86_64-apple-darwin" ;;
  *) echo "Unsupported CPU: $(uname -m)" >&2; exit 1 ;;
esac

URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ASSET}"

echo "→ iOS Runner v${VERSION} (${ASSET})"
mkdir -p "${HOME}/.ios-runner/bin"
curl -fsSL "$URL" -o "$INSTALL_BIN"
chmod +x "$INSTALL_BIN"

"$INSTALL_BIN" install-self
"$INSTALL_BIN" install-zed-tasks

echo ""
echo "✓ 已安装 CLI 与 Zed 全局任务。"
echo ""
echo "请在 Zed 中："
echo "  1. Extensions → 安装 iOS Runner（若尚未安装）"
echo "  2. Open Folder → 你的 iOS 工程目录"
echo "  3. Cmd+Shift+R 运行"
