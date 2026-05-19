# Bundled CLI (Zed extension)

macOS binaries shipped with the iOS-Runner Zed extension. On extension load they are copied to `~/.ios-runner/bin/ios-runner` (no network required).

Populated by:

```bash
./scripts/bundle-cli-for-extension.sh
```

Release tags run this before commit (see `scripts/release.sh`).
