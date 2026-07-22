#!/usr/bin/env bash
set -euo pipefail

# Release build. Extra args are forwarded to cargo, e.g. for a headless
# build without the GUI preview window:
#   ./build.sh --no-default-features
cargo build --release "$@"
