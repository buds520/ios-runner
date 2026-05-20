#!/usr/bin/env bash
# Smoke-test the bundled CocoaPods demo with the local debug CLI.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEMO="$ROOT/CocoaPodsDemo"
CLI="$ROOT/crates/target/debug/ios-runner"

if [[ ! -x "$CLI" ]]; then
  echo "Building local ios-runner CLI..."
  (cd "$ROOT/crates" && cargo build -p ios-runner-cli)
fi

if [[ ! -d "$DEMO/CocoaPodsDemo.xcodeproj" ]]; then
  if ! command -v xcodegen >/dev/null 2>&1; then
    echo "xcodegen is required to generate CocoaPodsDemo.xcodeproj." >&2
    echo "Install it with: brew install xcodegen" >&2
    exit 1
  fi
  (cd "$DEMO" && xcodegen generate)
fi

if [[ -f "$DEMO/Podfile" && ! -d "$DEMO/Pods" ]]; then
  if ! command -v pod >/dev/null 2>&1; then
    echo "CocoaPods is required for CocoaPodsDemo." >&2
    echo "Install it with: brew install cocoapods" >&2
    exit 1
  fi
  (cd "$DEMO" && pod install)
fi

echo "== ios-runner doctor =="
(cd "$DEMO" && "$CLI" doctor)

echo "== ios-runner ensure =="
(cd "$DEMO" && "$CLI" ensure)

echo "== ios-runner switch --list =="
(cd "$DEMO" && "$CLI" switch --list >/tmp/ios-runner-demo-destinations.json)
cat /tmp/ios-runner-demo-destinations.json

if [[ "${IOS_RUNNER_SMOKE_BUILD:-0}" == "1" ]]; then
  echo "== ios-runner build =="
  (cd "$DEMO" && "$CLI" build)
else
  echo "Skipping build. Set IOS_RUNNER_SMOKE_BUILD=1 to include xcodebuild."
fi

echo "✓ CocoaPodsDemo smoke test completed"
