#!/usr/bin/env bash
# One-shot install for Zed Dev Extension users (CLI + global Zed tasks).
set -euo pipefail

REPO_URL="${IOS_RUNNER_REPO:-https://github.com/buds520/ios-runner.git}"
SRC_DIR="${HOME}/.ios-runner/src/ios-runner"
INSTALL_BIN="${HOME}/.ios-runner/bin/ios-runner"

if [[ "$(uname -s)" != Darwin ]]; then
  echo "iOS Runner requires macOS." >&2
  exit 1
fi

for cmd in git cargo; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Missing dependency: $cmd" >&2
    exit 1
  fi
done

mkdir -p "${HOME}/.ios-runner/bin"

if [[ -d "$SRC_DIR/.git" ]]; then
  echo "→ Updating $SRC_DIR"
  git -C "$SRC_DIR" pull --ff-only
else
  echo "→ Cloning $REPO_URL → $SRC_DIR"
  mkdir -p "$(dirname "$SRC_DIR")"
  git clone "$REPO_URL" "$SRC_DIR"
fi

echo "→ Building CLI"
(cd "$SRC_DIR/crates" && cargo build -q -p ios-runner-cli --release)
cp "$SRC_DIR/crates/target/release/ios-runner" "$INSTALL_BIN"
chmod +x "$INSTALL_BIN"

echo "→ Installing Zed tasks and keymap"
"$INSTALL_BIN" install-zed-tasks

echo "→ Verifying"
"$INSTALL_BIN" doctor || true

echo ""
echo "✓ Dev install complete."
echo "  Next: Zed → Extensions → Install Dev Extension → select: $SRC_DIR"
echo "        File → Open Folder → your iOS project → Cmd+Shift+E / Cmd+Shift+R"
