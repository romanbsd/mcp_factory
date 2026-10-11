# mcp-gen

Python CLI for generating Rust MCP proxy servers from OpenAPI, GraphQL, and
Google API Discovery schemas.

Use `mcp-gen generate --input SCHEMA --output DIR` for one schema. Use
`mcp-gen compose --input SCHEMA_A --input SCHEMA_B --output DIR` to merge
several schemas into one server. Run `mcp-gen --help` or
`mcp-gen COMMAND --help` for all filtering, configuration, and packaging flags.

## Compose tool profiles

Pass `--tool-config tools.toml` to curate operations and expose named runtime profiles.
Profiles contain operation-ID globs; additive presets reuse them with `profile_extends`:

```toml
default_profiles = ["release_raw"]

[profiles]
core = ["apps_getCollection", "builds_getCollection"]
assets = ["appScreenshots_*"]
release_raw = []

[profile_extends]
release_raw = ["core", "assets"]

[custom_tools]
asset_upload = ["assets"]
```

`release_raw` receives both parents' operations and handwritten tool memberships.
Inheritance is transitive; child patterns are additive and duplicate patterns are removed.
Unknown profiles, cycles, reserved `all` inheritance and unmatched operation patterns fail
rather than silently reducing coverage. Existing configs without inheritance retain their
behavior. `all` remains the runtime preset exposing every configured tool.
