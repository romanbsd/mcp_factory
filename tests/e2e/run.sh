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
  --name e2e-openapi

"$VENV/bin/mcp-gen" generate \
  --input "$GENERATOR/tests/fixtures/minimal.graphql" \
  --output "$TMPDIR/graphql-server" \
  --base-url "http://127.0.0.1:1/graphql" \
  --name e2e-graphql

(
  cd "$TMPDIR/openapi-server"
  cargo check -q
)
(
  cd "$TMPDIR/graphql-server"
  cargo check -q
)

PKG_OUT="$TMPDIR/dist/petstore-mcp"
"$VENV/bin/mcp-gen" package \
  --input "$GENERATOR/tests/fixtures/minimal-openapi.yaml" \
  --output "$PKG_OUT" \
  --base-url "http://127.0.0.1:1" \
  --name e2e-openapi

test -x "$PKG_OUT/e2e-openapi"
test -f "$PKG_OUT/config.toml"
test -f "$PKG_OUT/README.txt"

echo "cross-layer e2e: ok"
