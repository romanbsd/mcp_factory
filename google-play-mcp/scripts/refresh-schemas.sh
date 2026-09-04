#!/usr/bin/env bash
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

curl -fsS 'https://androidpublisher.googleapis.com/$discovery/rest?version=v3' \
  -o "$server_root/schemas/androidpublisher-v3.json"
curl -fsS 'https://playdeveloperreporting.googleapis.com/$discovery/rest?version=v1beta1' \
  -o "$server_root/schemas/playdeveloperreporting-v1beta1.json"

"$generator" compose \
  --input "$server_root/schemas/androidpublisher-v3.json" \
  --input "$server_root/schemas/playdeveloperreporting-v1beta1.json" \
  --output "$server_root" \
  --name google-play-mcp \
  --config "$server_root/config.toml"
