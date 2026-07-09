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

## Use as an Agent Skill

This repo ships a root `SKILL.md`, so it works as an [Agent Skill](https://www.skills.sh/).
Install it into any skills-aware agent (Claude, Cursor, etc.):

```bash
npx skills add romanbsd/mcp_factory
```

## Quickstart

### Generate a server

```bash
cd generator
python3 -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"

`mcp-gen` auto-detects `crates/mcp-factory-core` from its install location (the repo checkout when using `pip install -e`). Pass `--core-path` only if the runtime crate lives elsewhere.

mcp-gen generate \
  --input tests/fixtures/minimal-openapi.yaml \
  --output ../examples/petstore-openapi \
  --base-url http://127.0.0.1:8080 \
  --name petstore-mcp
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

### Package a portable binary

Build a release binary and assemble a directory you can copy to another machine
(same OS/CPU as the build host, unless you pass `--target` for cross-compilation):

```bash
mcp-gen package \
  --input tests/fixtures/minimal-openapi.yaml \
  --output ../dist/petstore-mcp \
  --base-url http://127.0.0.1:8080 \
  --name petstore-mcp
```

This produces:

```text
dist/petstore-mcp/
  petstore-mcp    # release binary
  config.toml     # default upstream URL and transport
  README.txt      # Cursor config and env var reference
```

Copy the whole directory to the target machine, then point Cursor at the binary:

```json
{
  "mcpServers": {
    "petstore": {
      "command": "/path/on/target/petstore-mcp/petstore-mcp",
      "env": {
        "MCP_FACTORY_BASE_URL": "https://api.example.com"
      }
    }
  }
}
```

Optional flags:

| Flag | Description |
|------|-------------|
| `--archive` | Also write `dist/petstore-mcp.tar.gz` for `scp`/upload |
| `--target <triple>` | Cross-compile (e.g. `x86_64-unknown-linux-gnu`) |
| `--keep-source` | Keep generated Rust source as `dist/petstore-mcp-source/` |

#### Linux amd64 on Apple Silicon (Docker)

The easiest way to build a Linux x86_64 binary on an arm64 Mac is to run the
package step inside a `linux/amd64` container. No cross-linker or `--target`
setup required — only [Docker](https://docs.docker.com/get-docker/).

```bash
scripts/package-linux-amd64.sh \
  --input generator/tests/fixtures/minimal-openapi.yaml \
  --output dist/petstore-mcp-linux-amd64 \
  --base-url https://api.example.com \
  --name petstore-mcp \
  --archive
```

The script mounts the repo, installs `mcp-gen` in a `rust:bookworm` container,
and runs `mcp-gen package`. Output lands in `dist/` on your host.

Copy to the Linux host and extract:

```bash
scp dist/petstore-mcp-linux-amd64.tar.gz user@server:
ssh user@server 'tar xzf petstore-mcp-linux-amd64.tar.gz'
```

The first run downloads Rust crates and may take a few minutes.

<details>
<summary>Alternative: native cross-compile with <code>--target</code></summary>

If you prefer not to use Docker, install a Linux cross-linker and pass
`--target x86_64-unknown-linux-gnu` to `mcp-gen package` directly:

```bash
brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

mcp-gen package \
  --input tests/fixtures/minimal-openapi.yaml \
  --output ../dist/petstore-mcp-linux-amd64 \
  --base-url https://api.example.com \
  --name petstore-mcp \
  --target x86_64-unknown-linux-gnu \
  --archive
```

</details>

GraphQL SDL (`.graphql`/`.gql`) and GraphQL introspection JSON work the same way —
swap `--input` for your schema file. Swagger/OpenAPI 3.x YAML or JSON is auto-detected.

```bash
mcp-gen package \
  --input tests/fixtures/minimal.graphql \
  --output ../dist/graphql-mcp \
  --base-url http://127.0.0.1:8080/graphql \
  --name graphql-mcp \
  --archive
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

Each tool also carries hints derived from the schema, so the client sees more
than just a name and input schema:

- **`title`** from the OpenAPI `summary`.
- **`outputSchema`** from the first 2xx JSON response (OpenAPI) or the field's
  return type (GraphQL), so results can be validated and read field-by-field.
- **Annotations** from the operation: `readOnly`/`idempotent` for `GET`/`HEAD`
  and GraphQL queries, `destructive` for `DELETE`, `openWorld` always (it's an
  external API). Mutations and `POST`/`PATCH` are neither read-only nor
  idempotent.
- **Input schema** preserves `enum`, `format`, etc. (GraphQL enums become
  `{ "type": "string", "enum": [...] }`).

### Response mapping

The runtime adapts each upstream response into the richest MCP result it can:

| Upstream | MCP result |
|----------|------------|
| JSON body | text content **plus** `structuredContent` (the parsed JSON) |
| Non-JSON text | text content (decoded per the response charset) |
| Binary (image/audio/other) | `image` / `audio` / embedded blob resource, base64, with its MIME type |
| Large text (> 256 KiB) | embedded resource block instead of inline text |
| `204 No Content` | explicit success message |
| Non-2xx | tool error (`isError`) with `structuredContent` = `{ status, retryable, hint (401/403), retry_after, problem }` (RFC 7807 `problem+json` parsed when present) |

Useful response headers (`Location`, `Link`, `Retry-After`, `ETag`,
rate-limit, `Content-Range`) are surfaced under the result's `_meta` as
`http.<header>` so the client can chain requests and respect quotas.

## Configuration

Environment variables:

| Variable | Description |
|----------|-------------|
| `MCP_FACTORY_BASE_URL` | Upstream API base URL |
| `MCP_FACTORY_BEARER_TOKEN` | Bearer auth token |
| `MCP_FACTORY_API_KEY` | API key (header mode) |
| `MCP_FACTORY_OAUTH_CLIENT_SECRET` | OAuth2 client secret (optional, for confidential clients) |
| `MCP_TRANSPORT` | `stdio`, `http`, or `both` |
| `MCP_FACTORY_BIND_ADDR` | HTTP bind address (default `127.0.0.1:8080`) |

Generated crates also ship a `config.toml` template.

### OAuth2 (Authorization Code + PKCE)

For APIs that use OAuth2 instead of a static bearer token, configure `[auth]` with `type = "oauth2"` in `config.toml` (see the commented template in generated crates).

On a **stdio** server (the common case for local MCP clients like Cursor), the Authorization Code + PKCE flow launches **automatically** the first time a tool call needs a token: the server opens your browser, runs a loopback listener for the callback, stores the tokens, and completes the call. Subsequent calls reuse and silently refresh them. Progress is printed to stderr so it never corrupts the stdio protocol. (The very first call may exceed a client's tool-call timeout while you log in — just retry once tokens are stored, or pre-authenticate below.)

Auto-login is disabled for **HTTP** transport, where the server may be remote/headless; there you must log in ahead of time.

To log in ahead of time and store tokens locally (file mode `0600`, default `.mcp-factory/tokens.json`):

```bash
# Standalone CLI (from mcp-factory-core)
cargo run -p mcp-factory-core --bin mcp-factory-auth -- login --config config.toml

# Or via generated server
cargo run -- --auth-login
```

Check or clear stored tokens:

```bash
mcp-factory-auth status --config config.toml
mcp-factory-auth logout --config config.toml
```

The runtime refreshes access tokens automatically before upstream calls when a refresh token is present.

## Examples

- [`examples/petstore-openapi`](examples/petstore-openapi) — OpenAPI proxy with path/query/body params
- [`examples/graphql-example`](examples/graphql-example) — GraphQL query + mutation tools
