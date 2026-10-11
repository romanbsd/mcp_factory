#!/usr/bin/env python3
"""Measure actual stdio discovery without reading Apple credentials or calling Apple.
Run after cargo build -p app-store-connect-mcp. Optional --schemas exports default schemas.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--binary", type=Path, default=Path(__file__).resolve().parents[2] / "target/debug/app-store-connect-mcp")
parser.add_argument("--schemas", type=Path)
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
for profile in ("workflow", "core", "release_raw", "testflight_raw", "monetization_raw", "signing_raw", "reports_raw", "all"):
    env = {k: v for k, v in os.environ.items() if not k.startswith(("ASC_", "MCP_FACTORY_"))}
    env["MCP_FACTORY_PROFILES"] = profile
    process = subprocess.Popen([str(args.binary.resolve())], cwd=root, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
    def send(value):
        process.stdin.write(json.dumps(value) + "\n")
        process.stdin.flush()
    def receive(identifier):
        while True:
            line = process.stdout.readline()
            if not line:
                raise RuntimeError(f"{profile}: server terminated")
            value = json.loads(line)
            if value.get("id") == identifier:
                if "error" in value:
                    raise RuntimeError(value["error"])
                return value["result"]
    try:
        send({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"surface-measurement","version":"1"}}})
        receive(1)
        send({"jsonrpc":"2.0","method":"notifications/initialized"})
        send({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}})
        result = receive(2)
        names = [tool["name"] for tool in result["tools"]]
        assert names == sorted(names), "Tool listing must be sorted"
        send({"jsonrpc":"2.0","id":3,"method":"tools/list","params":{}})
        assert result == receive(3), "Repeated tool metadata changed"
        size = len(json.dumps(result, separators=(",", ":"), ensure_ascii=False).encode())
        print(json.dumps({"profile":profile,"tools":len(names),"compact_result_bytes":size}))
        if profile == "workflow" and args.schemas:
            args.schemas.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
    finally:
        process.stdin.close()
        process.terminate()
        process.wait(timeout=10)
