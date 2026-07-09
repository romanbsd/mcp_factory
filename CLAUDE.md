# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Two-part system that turns an OpenAPI 3.x or GraphQL schema into a runnable MCP proxy server:

- **`mcp-gen`** (Python, `generator/`) — parses a schema and emits a *thin* Rust crate.
- **`mcp-factory-core`** (Rust, `crates/mcp-factory-core/`) — the actual runtime. The generated crate just builds `ToolSpec`/`ResourceSpec` vectors and hands them to this library; all proxying logic lives here, not in generated code.

The generated crate is deliberately dumb (a `main.rs` + `tools.rs` + `resources.rs` of data). When changing behavior, change `mcp-factory-core` or the Jinja2 templates — almost never the generated output.

## Commands

Rust (workspace root):
```bash
cargo test
cargo clippy -- -D warnings          # warnings are errors; CI-equivalent gate
cargo run -p mcp-factory-core --bin mcp-factory-auth -- login --config config.toml
```

Python generator (`generator/`, needs venv with `pip install -e ".[dev]"`):
```bash
pytest tests --cov=mcp_gen --cov-fail-under=85   # coverage gate is enforced
pytest tests/unit/test_openapi.py -k some_name    # single test
UPDATE_GOLDEN=1 pytest tests/golden               # regenerate golden after template changes
```

Cross-layer e2e (generates a crate, then `cargo build`s it):
```bash
bash tests/e2e/run.sh
```

## The generation pipeline

`generator/mcp_gen/`:
1. `cli.py` — Typer app with `generate` and `package` subcommands. `resolve_core_path` (in `paths.py`) auto-detects the `mcp-factory-core` crate from the install location; `--core-path` overrides.
2. `openapi/parser.py` / `graphql/parser.py` — schema → `GenerationResult` (`models.py`: `ToolSpec`, `RestOperation`/`GraphQLOperation`, `ResourceSpec`). One tool per OpenAPI operation or GraphQL query/mutation field.
3. `render.py` — feeds `GenerationResult` into Jinja2 templates in `templates/` (`main.rs.j2`, `tools.rs.j2`, `resources.rs.j2`, `Cargo.toml.j2`, `config.toml.j2`).
4. `packaging.py` — `package` command additionally runs `cargo build --release` and assembles a portable dir (binary + `config.toml` + `README.txt`), optionally cross-compiled via `--target` or the Docker path in `scripts/package-linux-amd64.sh`.

**Templates are contract-tested.** `tests/golden/` holds expected rendered output; edits to any `.j2` file break golden tests until you run `UPDATE_GOLDEN=1`. Review the golden diff — it's the real review surface for template changes.

## The runtime (`mcp-factory-core/src/`)

- `server.rs` — `McpProxyServer` implements rmcp's `ServerHandler`. `call_tool` looks the tool up in the `ToolRegistry`, then dispatches on `ExecutionKind` to either `RestProxyExecutor` or `GraphQLProxyExecutor`. Built via a builder from a `ProxyConfig`.
- `tools/mod.rs` — `ToolSpec` carries an `ExecutionKind` enum (`Rest` | `GraphQL`); `ToolRegistry` rejects duplicate tool names.
- `rest/mod.rs` — binds MCP args into path/query/header/body via `ParamBinding`/`ParamLocation`, calls upstream with `reqwest`.
- `graphql/mod.rs` — sends a stored document + variable bindings.
- `config.rs` — `ProxyConfig::load(toml)` then `.merge_env()`; **env vars override the toml file**. `MCP_FACTORY_*` and `MCP_TRANSPORT` are the knobs (see README table).
- `transport/mod.rs` — stdio, streamable HTTP (axum), or both, selected by `TransportMode`.
- `resources/mod.rs` — exposes `schema://openapi`|`schema://graphql` and a `meta://tools` index.

### Auth (`auth/`)
Static bearer/API-key (from env) *or* OAuth2 Authorization Code + PKCE. `AuthProvider` trait abstracts the two; OAuth tokens persist to `.mcp-factory/tokens.json` (mode `0600`) via `token_store.rs` and auto-refresh before upstream calls. Login is interactive: `mcp-factory-auth login` (standalone bin) or `<generated-server> --auth-login`.

## Conventions

- Rust edition 2021, resolver 2. Shared deps are pinned in the workspace `Cargo.toml` `[workspace.dependencies]`; crate manifests use `.workspace = true`.
- Adding a runtime capability usually means: extend `mcp-factory-core`, then thread it through a template + regenerate golden. Test both layers.
