---
name: mcp-factory
description: >-
  Generate a runnable or portable MCP proxy from an OpenAPI 3.x or GraphQL
  schema (one tool per operation). Use when wrapping a REST or GraphQL API as
  MCP tools for Claude, Cursor, or another MCP client.
---

# MCP Factory

Python `mcp-gen` emits a thin Rust crate; `mcp-factory-core` does the proxying.
One MCP tool per OpenAPI operation or GraphQL query/mutation.

## Setup

Needs Rust (`cargo`) and Python ≥ 3.11:

```bash
cd generator
python3 -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"   # editable → auto-detects mcp-factory-core
```

## Generate or package

`generate` writes a crate you `cargo run`. `package` also `cargo build --release`s
and assembles `binary` + `config.toml` + `README.txt` (add `--archive` for `.tar.gz`).

Kind auto-detects (`.yaml`/`.yml`/OpenAPI JSON → openapi; `.graphql`/`.gql`/
introspection JSON → graphql). Override with `--kind`. `--base-url` defaults to
OpenAPI `servers[0].url` when present.

```bash
mcp-gen generate \
  --input path/to/schema.yaml \
  --output ./my-mcp \
  --base-url https://api.example.com \
  --name my-mcp
cd my-mcp && cargo run          # stdio MCP server
```

Same flags for `mcp-gen package --output ./dist/my-mcp`. Cross-compile with
`--target x86_64-unknown-linux-gnu`, or `scripts/package-linux-amd64.sh` (Docker).

| Flag | Effect |
|------|--------|
| `--transport stdio\|http\|both` | Transport (default `stdio`) |
| `--tags a,b` | OpenAPI: only these tags |
| `--include-deprecated` | Include deprecated ops |
| `--read-only` | GET/HEAD/OPTIONS + GraphQL queries only |
| `--core-path <dir>` | Override core crate path |

## Client wiring

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

HTTP: `MCP_TRANSPORT=http MCP_FACTORY_BIND_ADDR=127.0.0.1:8080 cargo run`.

## Config

Env vars override `config.toml`.

| Variable | Description |
|----------|-------------|
| `MCP_FACTORY_BASE_URL` | Upstream API base URL |
| `MCP_FACTORY_BEARER_TOKEN` | Bearer auth |
| `MCP_FACTORY_API_KEY` | API key (header) |
| `MCP_FACTORY_OAUTH_CLIENT_SECRET` | OAuth2 confidential client |
| `MCP_TRANSPORT` | `stdio`, `http`, or `both` |
| `MCP_FACTORY_BIND_ADDR` | HTTP bind (default `127.0.0.1:8080`) |

OAuth2 (Auth Code + PKCE): `<generated-server> --auth-login` (or
`mcp-factory-auth login --config config.toml`) → tokens in `.mcp-factory/tokens.json`.

## More

- Examples: `examples/petstore-openapi/`, `examples/graphql-example/`
- Fixtures: `generator/tests/fixtures/`
- Full reference: `README.md` · internals: `CLAUDE.md`
