#!/usr/bin/env bash
# Update buds520/extensions PR submodule + extensions.toml version for Zed marketplace review.
# Usage: ./scripts/update-zed-extensions-pr.sh [version] [extensions-repo-path]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="${1:-$(grep '^version = ' "$ROOT/extension.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')}"
EXTENSIONS_DIR="${2:-${EXTENSIONS_REPO:-$HOME/extensions}}"
BRANCH="${EXTENSIONS_BRANCH:-add-ios-runner}"
PR_NUMBER="${ZED_EXTENSIONS_PR:-6145}"

if [[ ! -d "$EXTENSIONS_DIR/.git" ]]; then
  echo "Extensions repo not found: $EXTENSIONS_DIR" >&2
  echo "Clone: git clone https://github.com/buds520/extensions.git $EXTENSIONS_DIR" >&2
  exit 1
fi

TAG="v${VERSION}"
echo "→ Updating Zed extensions PR (version ${VERSION}, tag ${TAG})"
echo "  extensions repo: $EXTENSIONS_DIR"

cd "$EXTENSIONS_DIR"
git fetch origin
git checkout "$BRANCH"
git pull origin "$BRANCH" || true

echo "→ Submodule extensions/ios-runner → ${TAG}"
git submodule update --init extensions/ios-runner
cd extensions/ios-runner
git fetch origin --tags
git checkout "$TAG" 2>/dev/null || git checkout "origin/main"
SUB_SHA="$(git rev-parse HEAD)"
cd "$EXTENSIONS_DIR"
git add extensions/ios-runner

if grep -q '^\[ios-runner\]' extensions.toml; then
  if sed --version 2>/dev/null | grep -q GNU; then
    sed -i "/^\[ios-runner\]/,/^\[/ s/^version = .*/version = \"${VERSION}\"/" extensions.toml
  else
    sed -i '' "/^\[ios-runner\]/,/^\[/ s/^version = .*/version = \"${VERSION}\"/" extensions.toml
  fi
else
  echo "[ios-runner] entry missing in extensions.toml" >&2
  exit 1
fi
git add extensions.toml

if command -v pnpm >/dev/null 2>&1 && [[ -f package.json ]]; then
  echo "→ pnpm sort-extensions"
  pnpm sort-extensions
  git add extensions.toml 2>/dev/null || true
fi

MSG="chore(ios-runner): bump to ${VERSION} (${SUB_SHA:0:7})"
if git diff --staged --quiet; then
  echo "No changes to commit in extensions repo."
else
  git commit -m "$MSG"
  git push origin "$BRANCH"
  echo "✓ Pushed to origin/${BRANCH}"
fi

if command -v gh >/dev/null 2>&1; then
  BODY="$(cat <<EOF
<!-- ios-runner-release-update -->
## iOS-Runner ${VERSION}

- Submodule: \`${SUB_SHA}\` ([buds520/ios-runner@${TAG}](https://github.com/buds520/ios-runner/releases/tag/${TAG}))
- \`extensions.toml\` version: \`${VERSION}\`

Automated update from [buds520/ios-runner](https://github.com/buds520/ios-runner) release workflow.
EOF
)"
  COMMENT_ID="$(gh api "repos/zed-industries/extensions/issues/${PR_NUMBER}/comments" \
    --jq '.[] | select(.body | contains("<!-- ios-runner-release-update -->")) | .id' \
    | tail -n 1 || true)"
  if [[ -n "$COMMENT_ID" ]]; then
    gh api --method PATCH "repos/zed-industries/extensions/issues/comments/${COMMENT_ID}" \
      -f body="$BODY" >/dev/null \
      || echo "⚠ Could not update PR #${PR_NUMBER} release comment (check gh auth)"
  else
    gh pr comment "$PR_NUMBER" \
      --repo zed-industries/extensions \
      --body "$BODY" || echo "⚠ Could not comment on PR #${PR_NUMBER} (check gh auth)"
  fi
  echo "PR: https://github.com/zed-industries/extensions/pull/${PR_NUMBER}"
fi
