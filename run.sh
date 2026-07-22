#!/usr/bin/env bash
set -euo pipefail

# Runs the release build. Extra args are forwarded to cargo, e.g. for a
# headless run without the GUI preview window:
#   ./run.sh --no-default-features
cargo run --release "$@"
