#!/usr/bin/env bash
# Full release: bump version → commit → tag → push → GitHub Release → update Zed extensions PR.
# Usage: ./scripts/release.sh 0.2.0 [--no-push] [--skip-extensions]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${1:?Usage: $0 <semver> e.g. 0.2.0 [--no-push] [--skip-extensions]}"
shift || true

DO_PUSH=1
SKIP_EXT=0
for arg in "$@"; do
  case "$arg" in
    --no-push) DO_PUSH=0 ;;
    --skip-extensions) SKIP_EXT=1 ;;
    *) echo "Unknown arg: $arg" >&2; exit 1 ;;
  esac
done

TAG="v${VERSION}"

if [[ -n "$(git status --porcelain)" ]]; then
  echo "→ Staging current changes for release commit"
  git add -A
fi

echo "→ Bump manifests to ${VERSION}"
"$ROOT/scripts/bump-version.sh" "$VERSION"

echo "→ Bundle macOS CLI into extension bin/ (offline install)"
chmod +x "$ROOT/scripts/bundle-cli-for-extension.sh"
"$ROOT/scripts/bundle-cli-for-extension.sh" "$VERSION"

echo "→ Sync embedded global Zed tasks for WASM extension"
chmod +x "$ROOT/scripts/sync-extension-embeds.sh"
"$ROOT/scripts/sync-extension-embeds.sh"

git add extension.toml Cargo.toml crates/Cargo.toml CHANGELOG.md bin/ src/embedded_global_tasks.json src/embedded_keymap_entry.json 2>/dev/null || true
git add -u

if git diff --staged --quiet && git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Tag ${TAG} already exists; skipping commit."
else
  git commit -m "chore: release ${TAG}" || echo "Nothing to commit (version already bumped?)"
fi

if ! git rev-parse "$TAG" >/dev/null 2>&1; then
  git tag -a "$TAG" -m "iOS-Runner ${VERSION}"
  echo "→ Created tag ${TAG}"
else
  echo "→ Tag ${TAG} exists"
fi

if [[ "$DO_PUSH" -eq 0 ]]; then
  echo "→ --no-push: local only. Run: git push origin main && git push origin ${TAG}"
  exit 0
fi

echo "→ git push"
git push origin HEAD
git push origin "$TAG"

echo "→ GitHub Release (CLI assets via Actions on tag push)"
if command -v gh >/dev/null 2>&1; then
  NOTES_FILE="$(mktemp)"
  if [[ -f CHANGELOG.md ]]; then
    awk "/^## \[?${VERSION}\]?/,/^## /{if (!/^## \[?${VERSION}/) print}" CHANGELOG.md | head -n 80 >"$NOTES_FILE" || true
  fi
  if [[ ! -s "$NOTES_FILE" ]]; then
    echo "See https://github.com/buds520/ios-runner/compare/${TAG}^...${TAG}" >"$NOTES_FILE"
  fi
  gh release view "$TAG" --repo buds520/ios-runner >/dev/null 2>&1 \
    && gh release edit "$TAG" --repo buds520/ios-runner --notes-file "$NOTES_FILE" \
    || gh release create "$TAG" --repo buds520/ios-runner --title "$TAG" --notes-file "$NOTES_FILE"
  rm -f "$NOTES_FILE"
  echo "✓ https://github.com/buds520/ios-runner/releases/tag/${TAG}"
else
  echo "Install gh CLI to create/edit release notes"
fi

if [[ "$SKIP_EXT" -eq 1 ]]; then
  echo "→ --skip-extensions: skipped Zed PR update"
else
  echo "→ Update Zed extensions marketplace PR"
  "$ROOT/scripts/update-zed-extensions-pr.sh" "$VERSION"
fi

echo ""
echo "Done. CI will attach macOS CLI binaries to the GitHub Release."
echo "Zed review PR: https://github.com/zed-industries/extensions/pull/${ZED_EXTENSIONS_PR:-6145}"
