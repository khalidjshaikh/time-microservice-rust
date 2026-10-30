#!/usr/bin/env bash
# Run the test suite.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

VERSION="${VERSION:-$(git describe --tags --always --dirty 2>/dev/null || echo dev)}"
COMMIT="${COMMIT:-$(git rev-parse --short HEAD 2>/dev/null || echo none)}"
BUILD_DATE="${BUILD_DATE:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"
export VERSION COMMIT BUILD_DATE

cargo test

# Produce an (empty) coverage artifact for Jenkins archiving.
: > coverage.out
echo ""
echo "Tests passed."
