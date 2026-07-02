#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GENERATOR="$ROOT/generator"
VENV="$GENERATOR/.venv"

if [[ ! -x "$VENV/bin/mcp-gen" ]]; then
  python3 -m venv "$VENV"
  "$VENV/bin/pip" install -e "$GENERATOR[dev]" -q
fi

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

"$VENV/bin/mcp-gen" generate \
  --input "$GENERATOR/tests/fixtures/minimal-openapi.yaml" \
  --output "$TMPDIR/openapi-server" \
  --base-url "http://127.0.0.1:1" \
  --name e2e-openapi \
  --core-path "$ROOT/crates/mcp-factory-core"

"$VENV/bin/mcp-gen" generate \
  --input "$GENERATOR/tests/fixtures/minimal.graphql" \
  --output "$TMPDIR/graphql-server" \
  --base-url "http://127.0.0.1:1/graphql" \
  --name e2e-graphql \
  --core-path "$ROOT/crates/mcp-factory-core"

(
  cd "$TMPDIR/openapi-server"
  cargo check -q
)
(
  cd "$TMPDIR/graphql-server"
  cargo check -q
)

echo "cross-layer e2e: ok"
