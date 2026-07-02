#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ $# -lt 1 ]]; then
  cat >&2 <<'EOF'
usage: scripts/package-linux-amd64.sh <mcp-gen package options>

Build a Linux amd64 MCP binary inside Docker (works on arm64 macOS).

example:
  scripts/package-linux-amd64.sh \
    --input generator/tests/fixtures/minimal-openapi.yaml \
    --output dist/petstore-mcp-linux-amd64 \
    --base-url https://api.example.com \
    --name petstore-mcp \
    --archive
EOF
  exit 1
fi

exec docker run --rm --platform linux/amd64 \
  -v "$ROOT:/work" \
  -w /work \
  rust:bookworm \
  bash /work/scripts/docker-mcp-gen-inner.sh package "$@"
