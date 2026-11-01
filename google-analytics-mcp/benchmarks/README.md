# Installed Google Analytics MCP comparison

Measured 2026-10-31 on this macOS host. Baseline is the existing uv tool
`analytics-mcp 0.8.0`, Python 3.15.2, through `~/.local/bin/analytics-mcp`.
Candidate is the workspace Rust release binary, version 0.1.0.
This is a historical 19-workflow snapshot, before the platform/stream selectors
and activation/retention workflows. The current default has 21 tools; rerun the
comparison below to measure that version.

Five alternating launches of each default server; medians below. Both received
identical MCP initialize and tools/list requests. RSS sampled 250 ms after the
listing. Rust used a temporary auth=none config. No credentials were read and no
Analytics API calls were made. Filesystem caches were warm; these are idle RSS
and startup measurements, not loaded reporting, peak memory or cold disk tests.

| Measurement | Installed Python | Rust workflow |
|---|---:|---:|
| Default tools | 9 | 19 |
| Tool listing, normalized UTF-8 JSON bytes | 55,115 | 16,239 |
| Idle RSS, MiB | 143.19 | 17.08 |
| Spawn through tools/list, ms | 553.60 | 8.00 |
| tools/list round trip, ms | 0.94 | 0.31 |

Rust used 88.1% less idle RSS and
70.5% less listing context.
Raw Rust profile (one run): 248 tools, 802,955 listing bytes, 34.22 MiB RSS.

The Python server's four report tools contribute 52,375 of its 55,115 listing
bytes: their descriptions embed substantial examples and repeated argument hints.
Rust's smaller descriptions and generic REST request bodies trade some inline
help for reduced context. Both list sizes include descriptions and schemas.
Token counts depend on the consuming model and were not measured.

The Python server offers annotations in its default profile and optional annotation
creation. Rust exposes these via raw generated operations, rather than dedicated
default workflows. Rust adds metadata search over standard definitions, Core
compatibility checks, batch/pivot reports, quota snapshots, configuration reads,
and access/change auditing. Its overview/acquisition/content/events/ecommerce
presets automate report construction; equivalent reports can also be assembled
with Python's generic run_report. Presets are convenience, not new API access.

Google's server returns pretty JSON text; Rust returns compact JSON text plus
structured content and preserves report metadata. Host handling of duplicate text
and structured content can affect output context. API response sizes, live data
parity and memory while reporting were not measured. Rust defaults bound report
rows; Python supports larger report pages. Configuration list snapshots may be
partial and explicitly report page tokens.

Reproduce with:

```sh
cargo build --release -p google-analytics-mcp
python3 google-analytics-mcp/scripts/compare.py
```

See `comparison.json` for individual samples and the `*-tools.json` files for
actual protocol tool inventories. The Python server reports MCP server version
1.0.0 in its handshake; its installed distribution version is 0.8.0.
