#!/usr/bin/env python3
"""Print the README coverage table: callable generated operations per profile,
counted by HTTP method from src/tools.rs (the generated source of truth)."""

import re
import sys
from collections import defaultdict
from pathlib import Path

root = Path(__file__).resolve().parent.parent
source = (root / "src" / "tools.rs").read_text(encoding="utf-8")

methods = {}
for block in source.split("ToolSpec {")[1:]:
    # The first `name:` in a block is the tool's; later ones are parameters.
    name = re.search(r'name: "([^"]+)"', block)
    method = re.search(r'method: "([A-Z]+)"', block)
    if name and method:
        methods[name.group(1)] = method.group(1)
profiles = re.findall(r'\("([^"]+)", &\[([^\]]*)\]\),', source)
if not methods or not profiles:
    sys.exit("could not read generated tools/profiles from src/tools.rs")

# Custom tools that implement a protocol on top of the generated operations.
uploads = {"assets", "monetization"}
counts = defaultdict(lambda: defaultdict(int))
for name, assigned in profiles:
    method = methods.get(name)
    for profile in re.findall(r'"([^"]+)"', assigned):
        if method:
            counts[profile][method] += 1

print("| Profile | Read (GET) | Create (POST) | Update (PATCH) | Delete (DELETE) | Upload |")
print("|---|---:|---:|---:|---:|:---:|")
for profile in ["core", "metadata", "assets", "testflight", "monetization", "pricing",
                "signing", "reviews", "reports"]:
    row = counts[profile]
    upload = "✓" if profile in uploads else "—"
    print(f"| `{profile}` | {row['GET']} | {row['POST']} | {row['PATCH']} | {row['DELETE']} | {upload} |")
total = defaultdict(int)
for method in methods.values():
    total[method] += 1
print(f"| **all** ({len(methods)} generated) | {total['GET']} | {total['POST']} | "
      f"{total['PATCH']} | {total['DELETE']} | ✓ |")
