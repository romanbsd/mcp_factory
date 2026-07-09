---
name: mcp-factory
description: Generate and run an MCP proxy server from an OpenAPI 3.x or GraphQL schema. Use when someone wants to expose a REST/GraphQL API to an MCP client (Claude, Cursor, etc.) as tools — turning a schema (OpenAPI YAML/JSON or GraphQL SDL/introspection) into a runnable or portable MCP server. Triggers: "MCP server from OpenAPI", "wrap this API as MCP tools", "GraphQL to MCP", "generate an MCP proxy".
---

# MCP Factory

Turns an OpenAPI 3.x or GraphQL schema into a Rust MCP proxy server. One tool per
OpenAPI operation / GraphQL query/mutation field. Proxying logic lives in the
`mcp-factory-core` Rust runtime; the Python `mcp-gen` CLI emits a thin generated crate.

## Prerequisites

- **Rust** toolchain (`cargo`) — to build/run the generated server.
- **Python ≥ 3.11** — for `mcp-gen`.

Install the CLI from this repo (editable — lets `mcp-gen` auto-detect the core crate):

```bash
cd generator
python3 -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
```

## Two commands

`mcp-gen generate` emits a Rust crate you `cargo run`. `mcp-gen package` also runs
`cargo build --release` and assembles a portable dir (binary + `config.toml` + `README.txt`).

Schema kind auto-detects from extension/content (`.yaml`/`.yml`/`.json` = OpenAPI,
`.graphql`/`.gql` or introspection JSON = GraphQL). Override with `--kind`.

### Generate + run (dev)

```bash
mcp-gen generate \
  --input path/to/schema.yaml \
  --output ./my-mcp \
  --base-url https://api.example.com \
  --name my-mcp
cd my-mcp && cargo run          # stdio MCP server
```

### Package a portable binary

```bash
mcp-gen package \
  --input path/to/schema.yaml \
  --output ./dist/my-mcp \
  --base-url https://api.example.com \
  --name my-mcp \
  --archive                     # also writes dist/my-mcp.tar.gz
```

Cross-compile with `--target x86_64-unknown-linux-gnu` (needs a cross-linker), or use
`scripts/package-linux-amd64.sh` for Linux amd64 via Docker. See README.md.

### Common flags

| Flag | Effect |
|------|--------|
| `--transport stdio\|http\|both` | Transport (default `stdio`) |
| `--tags a,b` | OpenAPI: only ops with these tags |
| `--include-deprecated` | Include deprecated operations |
| `--core-path <dir>` | Point at `mcp-factory-core` if not auto-detected |

## Wiring into a client

Point the MCP client at the built binary. Cursor / Claude Desktop:

```json
{
  "mcpServers": {
    "my-mcp": {
      "command": "/abs/path/to/my-mcp/target/debug/my-mcp",
      "env": { "MCP_FACTORY_BASE_URL": "https://api.example.com" }
    }
  }
}
```

HTTP transport instead of stdio: `MCP_TRANSPORT=http MCP_FACTORY_BIND_ADDR=127.0.0.1:8080 cargo run`.

## Configuration (env vars override `config.toml`)

| Variable | Description |
|----------|-------------|
| `MCP_FACTORY_BASE_URL` | Upstream API base URL |
| `MCP_FACTORY_BEARER_TOKEN` | Bearer auth token |
| `MCP_FACTORY_API_KEY` | API key (header mode) |
| `MCP_FACTORY_OAUTH_CLIENT_SECRET` | OAuth2 client secret (confidential clients) |
| `MCP_TRANSPORT` | `stdio`, `http`, or `both` |
| `MCP_FACTORY_BIND_ADDR` | HTTP bind address (default `127.0.0.1:8080`) |

**OAuth2 (Auth Code + PKCE):** interactive login persists tokens to
`.mcp-factory/tokens.json` and auto-refreshes. Run `<generated-server> --auth-login`
(or the standalone `mcp-factory-auth login --config config.toml`).

## What the generated server exposes

- **Tools** — one per operation, with `title`, `outputSchema`, and annotations
  (`readOnly`/`idempotent` for GET/queries, `destructive` for DELETE) derived from the schema.
- **Resources** — `schema://openapi` | `schema://graphql`, plus a `meta://tools` index.
- **Responses** adapted to richest MCP result: JSON → text + `structuredContent`; binary →
  image/audio/blob; non-2xx → tool error with `{ status, retryable, hint, retry_after, problem }`.
  Useful headers (`Location`, `Retry-After`, `ETag`, rate-limit) surface under `_meta`.

## Examples & depth

- Worked examples: `examples/petstore-openapi/`, `examples/graphql-example/`.
- Fixtures to try: `generator/tests/fixtures/`.
- Full reference: `README.md`. Architecture / internals: `CLAUDE.md`.
