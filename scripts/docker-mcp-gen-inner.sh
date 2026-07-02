#!/usr/bin/env bash
set -euo pipefail

apt-get update -qq
DEBIAN_FRONTEND=noninteractive apt-get install -y -qq python3 python3-pip python3-venv >/dev/null

python3 -m venv /tmp/venv
/tmp/venv/bin/pip install -q -e "/work/generator[dev]"
exec /tmp/venv/bin/mcp-gen "$@"
