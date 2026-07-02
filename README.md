# MCP Factory

Generate and run MCP proxy servers from OpenAPI 3.x or GraphQL schemas.

## Architecture

- **`mcp-factory-core`** (Rust): rmcp-based runtime that proxies tool calls to REST/GraphQL backends over stdio or Streamable HTTP.
- **`mcp-gen`** (Python): parses schemas and emits thin generated Rust crates.

```text
OpenAPI / GraphQL schema
        │
        ▼
     mcp-gen (Python)
        │
        ▼
 generated Rust crate ──► mcp-factory-core ──► upstream API
```

## Quickstart

### Generate a server

```bash
cd generator
python3 -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"

mcp-gen generate \
  --input tests/fixtures/minimal-openapi.yaml \
  --output ../examples/petstore-openapi \
  --base-url http://127.0.0.1:8080 \
  --name petstore-mcp \
  --core-path ../../crates/mcp-factory-core
```

### Run generated server (stdio)

```bash
cd examples/petstore-openapi
cargo run
```

Configure Cursor MCP with:

```json
{
  "mcpServers": {
    "petstore": {
      "command": "/path/to/examples/petstore-openapi/target/debug/petstore-mcp"
    }
  }
}
```

### HTTP transport

```bash
MCP_TRANSPORT=http MCP_FACTORY_BIND_ADDR=127.0.0.1:8080 cargo run
```

## Testing

```bash
# Rust
cargo test
cargo clippy -- -D warnings

# Python
cd generator
source .venv/bin/activate
pytest tests --cov=mcp_gen --cov-fail-under=85

# Update golden files after intentional template changes
UPDATE_GOLDEN=1 pytest tests/golden

# Cross-layer e2e
bash tests/e2e/run.sh
```

## Generated MCP surface

| Primitive | Source |
|-----------|--------|
| Tools | One per OpenAPI operation or GraphQL query/mutation field |
| Resources | `schema://openapi` or `schema://graphql`, plus `meta://tools` index |

## Configuration

Environment variables:

| Variable | Description |
|----------|-------------|
| `MCP_FACTORY_BASE_URL` | Upstream API base URL |
| `MCP_FACTORY_BEARER_TOKEN` | Bearer auth token |
| `MCP_FACTORY_API_KEY` | API key (header mode) |
| `MCP_TRANSPORT` | `stdio`, `http`, or `both` |
| `MCP_FACTORY_BIND_ADDR` | HTTP bind address (default `127.0.0.1:8080`) |

Generated crates also ship a `config.toml` template.

## Examples

- [`examples/petstore-openapi`](examples/petstore-openapi) — OpenAPI proxy with path/query/body params
- [`examples/graphql-example`](examples/graphql-example) — GraphQL query + mutation tools
