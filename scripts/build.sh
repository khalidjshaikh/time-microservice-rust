#!/usr/bin/env bash
# Build the time-microservice-rust binary.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

mkdir -p bin

VERSION="${VERSION:-$(git describe --tags --always --dirty 2>/dev/null || echo dev)}"
COMMIT="${COMMIT:-$(git rev-parse --short HEAD 2>/dev/null || echo none)}"
BUILD_DATE="${BUILD_DATE:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"

export VERSION COMMIT BUILD_DATE

cargo build --release
cp target/release/time-microservice-rust bin/time-microservice-rust
chmod +x bin/time-microservice-rust
echo "Built bin/time-microservice-rust"
