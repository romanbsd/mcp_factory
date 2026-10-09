#!/usr/bin/env bash
# Regenerate the crate from openapi.oas.json. tools.toml selects the operations
# and assigns them to runtime profiles; response schemas are omitted (they are
# hundreds of KB of tools/list) and JSON:API link boilerplate is compacted.
set -euo pipefail

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
server_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
factory_root=$(CDPATH= cd -- "$server_root/.." && pwd)
generator="$factory_root/generator/.venv/bin/mcp-gen"

if [[ ! -x "$generator" ]]; then
  echo "mcp-gen is not installed at $generator" >&2
  echo "Run the setup steps in $factory_root/SKILL.md first." >&2
  exit 1
fi

"$generator" generate \
  --input "$server_root/openapi.oas.json" \
  --output "$server_root" \
  --name app-store-connect-mcp \
  --no-output-schema \
  --compact-jsonapi \
  --tool-config "$server_root/tools.toml"
